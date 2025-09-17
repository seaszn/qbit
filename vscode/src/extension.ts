import * as vscode from 'vscode';
import { HoverProvider } from './hover';
import { DiagnosticsProvider } from './diagnostics';
import { CompletionProvider } from './completion';

export function activate(context: vscode.ExtensionContext) {
    console.log('Qbit language extension is now active!');

    const qbitSelector: vscode.DocumentSelector = {
        scheme: 'file',
        language: 'qbit'
    };

    const diagnosticsProvider = new DiagnosticsProvider();
    
    context.subscriptions.push(
        vscode.workspace.onDidChangeTextDocument(event => {
            if (event.document.languageId === 'qbit') {
                diagnosticsProvider.updateDiagnostics(event.document);
            }
        }),
        vscode.workspace.onDidOpenTextDocument(document => {
            if (document.languageId === 'qbit') {
                diagnosticsProvider.updateDiagnostics(document);
            }
        }),
        vscode.workspace.onDidCloseTextDocument(document => {
            if (document.languageId === 'qbit') {
                diagnosticsProvider.clearDiagnostics(document);
            }
        }),
        vscode.languages.registerHoverProvider(qbitSelector, new HoverProvider()),
        vscode.languages.registerCompletionItemProvider(
            qbitSelector,
            new CompletionProvider(),
            '.', '('
        ),
        vscode.commands.registerCommand('qbit.helloWorld', () => {
            vscode.window.showInformationMessage('Hello from Qbit!');
        })
    );
}

export function deactivate() {}