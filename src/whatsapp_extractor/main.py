import threading
import toga
from toga.style import Pack
from whatsapp_extractor.app import app

class WhatsAppExtractorApp(toga.App):
    def startup(self):
        # Start Flask server in a background daemon thread
        self.flask_thread = threading.Thread(target=self.run_flask, daemon=True)
        self.flask_thread.start()

        # Create main window
        self.main_window = toga.MainWindow(title=self.formal_name)

        # Create WebView pointing to the local Flask app
        self.webview = toga.WebView(
            url="http://127.0.0.1:5000/",
            style=Pack(flex=1)
        )

        self.main_window.content = self.webview
        self.main_window.show()

    def run_flask(self):
        # Disable logging and run local server
        import logging
        log = logging.getLogger('werkzeug')
        log.setLevel(logging.ERROR)
        app.run(host="127.0.0.1", port=5000, debug=False, use_reloader=False)

def main():
    return WhatsAppExtractorApp("WhatsApp Extractor", "com.daferferso.whatsapp_extractor")
