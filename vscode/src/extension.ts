import * as vscode from 'vscode';
import { HoverProvider } from './hover';
import { DiagnosticsProvider } from './diagnostics';
import { CompletionProvider } from './completion';

export function activate(context: vscode.ExtensionContext) {
    const selector: vscode.DocumentSelector = {
        scheme: 'file',
        language: 'qbit'
    };

    const provider = new DiagnosticsProvider();
    
    context.subscriptions.push(
        vscode.workspace.onDidChangeTextDocument(event => {
            if (event.document.languageId === 'qbit') {
                provider.update(event.document);
            }
        }),
        vscode.workspace.onDidOpenTextDocument(document => {
            if (document.languageId === 'qbit') {
                provider.update(document);
            }
        }),
        vscode.workspace.onDidCloseTextDocument(document => {
            if (document.languageId === 'qbit') {
                provider.clear(document);
            }
        }),
        vscode.languages.registerHoverProvider(selector, new HoverProvider()),
        vscode.languages.registerCompletionItemProvider(
            selector,
            new CompletionProvider(),
            '.', '('
        ),
        vscode.commands.registerCommand('qbit.helloWorld', () => {
            vscode.window.showInformationMessage('Hello from Qbit!');
        })
    );
}

export function deactivate() {}