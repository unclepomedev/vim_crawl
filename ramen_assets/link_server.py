import os
import socket
import threading
import traceback

import hdefereval
import hou

MAX_SCRIPT_SIZE = 10 * 1024 * 1024  # 10 MB
HOUDINI_RAMEN_TOKEN = os.getenv("HOUDINI_RAMEN_TOKEN")
if not HOUDINI_RAMEN_TOKEN:
    raise RuntimeError("HOUDINI_RAMEN_TOKEN is not set")
try:
    LIVE_LINK_PORT = int(os.getenv("HOUDINI_RAMEN_PORT", "18080"))
except ValueError as err:
    raise RuntimeError("HOUDINI_RAMEN_PORT must be a valid integer") from err
if not 0 <= LIVE_LINK_PORT <= 65535:
    raise RuntimeError("HOUDINI_RAMEN_PORT must be between 0 and 65535")


class HoudiniLiveLinkServer:
    def __init__(self, host="127.0.0.1", port=LIVE_LINK_PORT):
        self.host = host
        self.port = port
        self.server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        try:
            self.server_socket.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
            self.server_socket.bind((self.host, self.port))
        except OSError:
            self.server_socket.close()
            raise
        self._stop_event = threading.Event()
        self._server_thread = None

    def start(self, daemon=True):
        def run():
            self._run_server()

        self._server_thread = threading.Thread(target=run, daemon=daemon)
        self._server_thread.start()

    def _run_server(self):
        self.server_socket.listen(1)
        print(f"🍜 Houdini Ramen: Listening on {self.host}:{self.port}...")
        self.server_socket.settimeout(1.0)

        while not self._stop_event.is_set():
            try:
                client, _addr = self.server_socket.accept()
                client.settimeout(5.0)
                try:
                    self._handle_client(client)
                finally:
                    client.close()
            except socket.timeout:
                continue
            except OSError as e:
                if not self._stop_event.is_set():
                    print(f"❌ Server error: {e}")

    def _handle_client(self, client):
        chunks = []
        total = 0
        is_oversize = False

        try:
            while True:
                packet = client.recv(4096)
                if not packet:
                    break
                chunks.append(packet)
                total += len(packet)
                if total > MAX_SCRIPT_SIZE:
                    is_oversize = True
                    break
        except socket.timeout:
            client.sendall(b"ERROR\nServer read timeout.")
            return

        if is_oversize:
            print("❌ Received data exceeds maximum allowed size, dropping.")
            client.sendall(b"ERROR\nReceived data exceeds maximum allowed size.")
            return

        if not chunks:
            client.sendall(b"ERROR\nReceived empty script.")
            return

        try:
            payload = b"".join(chunks).decode("utf-8")
        except UnicodeDecodeError:
            client.sendall(b"ERROR\nInvalid UTF-8 encoding in payload.")
            return

        auth_prefix = f"AUTH:{HOUDINI_RAMEN_TOKEN}\n"
        if not payload.startswith(auth_prefix):
            print("❌ Unauthorized connection attempt rejected.")
            client.sendall(b"ERROR\nUnauthorized payload. Access denied.")
            return
        script = payload[len(auth_prefix) :]

        print("✅ Received valid script from Rust, executing...")
        response = self._execute_in_houdini(script)
        client.sendall(response)

    @staticmethod
    def _execute_in_houdini(script):
        def task():
            try:
                exec(script, {"hou": hou, "__builtins__": __builtins__})
                return b"OK"
            except Exception:
                return f"ERROR\n{traceback.format_exc()}".encode("utf-8")

        try:
            return hdefereval.executeInMainThreadWithResult(task)
        except Exception as e:
            return f"ERROR\nFailed to schedule execution: {e}".encode("utf-8")

    def stop(self, timeout=1.0):
        self._stop_event.set()
        try:
            self.server_socket.shutdown(socket.SHUT_RDWR)
        except OSError:
            pass
        self.server_socket.close()

        if self._server_thread and self._server_thread.is_alive():
            self._server_thread.join(timeout=timeout)
            if self._server_thread.is_alive():
                print("⚠️ Houdini Ramen: Server thread did not exit in time.")


if hasattr(hou.session, "ramen_server"):
    hou.session.ramen_server.stop()

try:
    server = HoudiniLiveLinkServer()
    hou.session.ramen_server = server
    server.start(daemon=True)
except OSError as err:
    print(
        f"❌ Houdini Ramen: Failed to start live-link server on port {LIVE_LINK_PORT}: {err}"
    )
