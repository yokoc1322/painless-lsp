/* --------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See License.txt in the project root for license information.
 * ------------------------------------------------------------------------------------------ */

import * as os from 'os';
import {
  languages,
  workspace,
  EventEmitter,
  ExtensionContext,
  window,
  InlayHintsProvider,
  TextDocument,
  CancellationToken,
  Range,
  InlayHint,
  TextDocumentChangeEvent,
  ProviderResult,
  commands,
  WorkspaceEdit,
  TextEdit,
  Selection,
  Uri,
} from "vscode";

import {
  Disposable,
  Executable,
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
} from "vscode-languageclient/node";

let client: LanguageClient;

export async function activate(context: ExtensionContext) {

  const traceOutputChannel = window.createOutputChannel("Painless Language Server trace");
  const serverArchDir = getServerAppDir();
  const command = process.env.SERVER_PATH || Uri.joinPath(context.extensionUri, "target", serverArchDir, "release", "painless-lsp").fsPath;
  const run: Executable = {
    command,
    options: {
      env: {
        ...process.env,
        RUST_LOG: "debug",
        RUST_BACKTRACE: "1",
      },
    },
  };
  const serverOptions: ServerOptions = {
    run,
    debug: run,
  };
  let clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: "file", language: "painless" }],
    synchronize: {
      fileEvents: workspace.createFileSystemWatcher("**/.painless"),
    },
    traceOutputChannel,
  };

  client = new LanguageClient("painless-language-server", "painless language server", serverOptions, clientOptions);
  client.start();
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  return client.stop();
}

function getServerAppDir(): PlatformType {
  const osType = os.type();
  const osArch = os.arch();
  const errorMessage = "Unsupported platform: " + osType + " " + osArch;

  switch (osType) {
    case "Windows_NT":
      switch (osArch) {
        case "x64":
          return PlatformType.WINDOWS_X86_64;
        default:
          throw new Error(errorMessage);
      }
    case "Linux":
      switch (osArch) {
        case "x64":
          return PlatformType.LINUX_X86_64;
        default:
          throw new Error(errorMessage);
      }
    case "Darwin":
      switch (osArch) {
        case "arm64":
          return PlatformType.MACOS_ARM64;
        default:
          throw new Error(errorMessage);
      }
    default:
      throw new Error(errorMessage);
  }
}

enum PlatformType {
  WINDOWS_X86_64 = "windows-x86-64",
  LINUX_X86_64 = "linux-x86-64",
  MACOS_ARM64 = "macos-arm64",
}
