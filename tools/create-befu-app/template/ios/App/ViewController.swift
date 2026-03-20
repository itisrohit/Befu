import Foundation
import UIKit
import WebKit

private let befuBridgeHandler = "befuNative"

@MainActor
final class ViewController: UIViewController, WKScriptMessageHandler, WKUIDelegate {
    private let webView: WKWebView

    init() {
        let contentController = WKUserContentController()
        let bridgeScript = """
        (function () {
          const pending = new Map();
          window.__befuNativeResolve = function (id, payloadJson) {
            const resolve = pending.get(id);
            if (!resolve) return;
            pending.delete(id);
            resolve(payloadJson);
          };

          window.BefuNative = {
            invokeRaw(payloadJson) {
              try {
                const payload = JSON.parse(payloadJson);
                return new Promise((resolve) => {
                  pending.set(payload.id, resolve);
                  window.webkit.messageHandlers.befuNative.postMessage(payloadJson);
                });
              } catch (_error) {
                return Promise.resolve(JSON.stringify({
                  id: "",
                  ok: false,
                  error: { code: "INVALID_JSON", message: "Invalid payload JSON" },
                }));
              }
            },
            backendMode() {
              return "ios";
            },
          };
        })();
        """
        contentController.addUserScript(
            WKUserScript(source: bridgeScript, injectionTime: .atDocumentStart, forMainFrameOnly: true)
        )

        let config = WKWebViewConfiguration()
        config.userContentController = contentController

        self.webView = WKWebView(frame: .zero, configuration: config)
        super.init(nibName: nil, bundle: nil)

        contentController.add(self, name: befuBridgeHandler)
        // Required so JS alert(), confirm(), prompt() work inside WKWebView
        self.webView.uiDelegate = self
    }

    deinit {
        MainActor.assumeIsolated {
            webView.configuration.userContentController.removeScriptMessageHandler(forName: befuBridgeHandler)
        }
    }

    @available(*, unavailable)
    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }

    override func loadView() {
        view = webView
    }

    override func viewDidLoad() {
        super.viewDidLoad()

        #if DEBUG
            if let url = URL(string: "http://localhost:5173") {
                webView.load(URLRequest(url: url))
                return
            }
        #endif

        if let fileURL = Bundle.main.url(forResource: "index", withExtension: "html", subdirectory: "web") {
            webView.loadFileURL(fileURL, allowingReadAccessTo: fileURL.deletingLastPathComponent())
            return
        }

        webView.loadHTMLString(
            "<html><body><h3>Befu iOS shell</h3><p>Release web assets not found.</p></body></html>",
            baseURL: nil
        )
    }

    func userContentController(_ userContentController: WKUserContentController, didReceive message: WKScriptMessage) {
        guard message.name == befuBridgeHandler,
              let payloadJson = message.body as? String
        else {
            return
        }

        guard let id = requestId(from: payloadJson) else {
            return
        }

        let responseJson = invokeRustCore(payloadJson)

        guard let encodedIdLiteral = jsonStringLiteral(id),
              let encodedResponseLiteral = jsonStringLiteral(responseJson)
        else {
            return
        }

        let js = "window.__befuNativeResolve(\(encodedIdLiteral), \(encodedResponseLiteral))"
        webView.evaluateJavaScript(js, completionHandler: nil)
    }

    private func requestId(from payloadJson: String) -> String? {
        guard let data = payloadJson.data(using: .utf8),
              let object = try? JSONSerialization.jsonObject(with: data) as? [String: Any],
              let id = object["id"] as? String
        else {
            return nil
        }

        return id
    }

    private func invokeRustCore(_ payloadJson: String) -> String {
        let cStringBytes = payloadJson.utf8CString
        return cStringBytes.withUnsafeBufferPointer { buffer in
            guard let payloadPointer = buffer.baseAddress,
                  let responsePointer = befu_invoke_raw(payloadPointer)
            else {
                return "{\"id\":\"\",\"ok\":false,\"error\":{\"code\":\"NATIVE_BRIDGE_FAILURE\",\"message\":\"Rust bridge invocation failed\"}}"
            }

            defer { befu_free_string(responsePointer) }
            return String(cString: responsePointer)
        }
    }

    private func jsonStringLiteral(_ value: String) -> String? {
        guard let data = try? JSONEncoder().encode(value),
              let encoded = String(data: data, encoding: .utf8)
        else {
            return nil
        }

        return encoded
    }

    // MARK: - WKUIDelegate (enables JS alert/confirm/prompt)

    func webView(
        _ webView: WKWebView,
        runJavaScriptAlertPanelWithMessage message: String,
        initiatedByFrame frame: WKFrameInfo,
        completionHandler: @escaping () -> Void
    ) {
        let alert = UIAlertController(title: nil, message: message, preferredStyle: .alert)
        alert.addAction(UIAlertAction(title: "OK", style: .default) { _ in completionHandler() })
        present(alert, animated: true)
    }

    func webView(
        _ webView: WKWebView,
        runJavaScriptConfirmPanelWithMessage message: String,
        initiatedByFrame frame: WKFrameInfo,
        completionHandler: @escaping (Bool) -> Void
    ) {
        let alert = UIAlertController(title: nil, message: message, preferredStyle: .alert)
        alert.addAction(UIAlertAction(title: "Cancel", style: .cancel) { _ in completionHandler(false) })
        alert.addAction(UIAlertAction(title: "OK", style: .default) { _ in completionHandler(true) })
        present(alert, animated: true)
    }

    func webView(
        _ webView: WKWebView,
        runJavaScriptTextInputPanelWithPrompt prompt: String,
        defaultText: String?,
        initiatedByFrame frame: WKFrameInfo,
        completionHandler: @escaping (String?) -> Void
    ) {
        let alert = UIAlertController(title: nil, message: prompt, preferredStyle: .alert)
        alert.addTextField { tf in tf.text = defaultText }
        alert.addAction(UIAlertAction(title: "Cancel", style: .cancel) { _ in completionHandler(nil) })
        alert.addAction(UIAlertAction(title: "OK", style: .default) { _ in
            completionHandler(alert.textFields?.first?.text)
        })
        present(alert, animated: true)
    }
}
