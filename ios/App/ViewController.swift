import Foundation
import UIKit
import WebKit

private let befuBridgeHandler = "befuNative"

private struct BridgeErrorDetail: Encodable {
    let code: String
    let message: String
}

private struct BridgeResponseEnvelope<ResultType: Encodable>: Encodable {
    let id: String
    let ok: Bool
    let result: ResultType?
    let error: BridgeErrorDetail?
}

private struct PingResult: Encodable {
    let pong: String
}

private struct AppInfoResult: Encodable {
    let name: String
    let version: String
    let runtime: String
}

@MainActor
final class ViewController: UIViewController, WKScriptMessageHandler {
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

        let responseJson = handleFallbackInvokeRaw(payloadJson)
        guard let id = requestId(from: payloadJson) else {
            return
        }

        guard let encodedResponseLiteral = jsonStringLiteral(responseJson) else {
            return
        }

        let js = "window.__befuNativeResolve(\"\(id)\", \(encodedResponseLiteral))"
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

    private func handleFallbackInvokeRaw(_ payloadJson: String) -> String {
        guard let data = payloadJson.data(using: .utf8),
              let object = try? JSONSerialization.jsonObject(with: data) as? [String: Any],
              let id = object["id"] as? String,
              let command = object["command"] as? String
        else {
            return encodeResponse(
                BridgeResponseEnvelope<EmptyResult>(
                    id: "",
                    ok: false,
                    result: nil,
                    error: BridgeErrorDetail(code: "INVALID_JSON", message: "Invalid payload JSON")
                )
            )
        }

        switch command {
        case "ping":
            return encodeResponse(
                BridgeResponseEnvelope(
                    id: id,
                    ok: true,
                    result: PingResult(pong: "pong"),
                    error: nil
                )
            )
        case "app.info":
            let version = Bundle.main.object(forInfoDictionaryKey: "CFBundleShortVersionString") as? String ?? "0.1.0"
            return encodeResponse(
                BridgeResponseEnvelope(
                    id: id,
                    ok: true,
                    result: AppInfoResult(name: "Befu", version: version, runtime: "befu"),
                    error: nil
                )
            )
        default:
            return encodeResponse(
                BridgeResponseEnvelope<EmptyResult>(
                    id: id,
                    ok: false,
                    result: nil,
                    error: BridgeErrorDetail(code: "UNKNOWN_COMMAND", message: "Unknown command: \(command)")
                )
            )
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

    private func encodeResponse<ResponseType: Encodable>(_ response: ResponseType) -> String {
        guard let data = try? JSONEncoder().encode(response),
              let json = String(data: data, encoding: .utf8)
        else {
            return "{\"id\":\"\",\"ok\":false,\"error\":{\"code\":\"ENCODING_FAILURE\",\"message\":\"Failed to encode response\"}}"
        }

        return json
    }
}

private struct EmptyResult: Encodable {}
