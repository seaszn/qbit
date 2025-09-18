import * as vscode from 'vscode';
import { Parser } from './parser';

export class DiagnosticsProvider {
    private diagnosticCollection: vscode.DiagnosticCollection;

    constructor() {
        this.diagnosticCollection = vscode.languages.createDiagnosticCollection('qbit');
    }

    async update(document: vscode.TextDocument): Promise<void> {
        if (document.languageId !== 'qbit') {
            return;
        }

        try {
            const errors = await Parser.parse(document.getText());

            const diagnostics: vscode.Diagnostic[] = errors.diagnositcs.map(error => {
                const range = new vscode.Range(
                    new vscode.Position(Math.max(0, error.line - 1), Math.max(0, error.column - 1)),
                    new vscode.Position(Math.max(0, error.line - 1), Math.max(0, error.column - 1 + error.length))
                );

                const diagnostic = new vscode.Diagnostic(
                    range,
                    error.message,
                    error.level
                );

                diagnostic.source = 'qbit';
                return diagnostic;
            });

            this.diagnosticCollection.set(document.uri, diagnostics);
        } catch (err) {
            console.error('Error updating diagnostics:', err);
            this.diagnosticCollection.set(document.uri, []);
        }
    }

    clear(document: vscode.TextDocument): void {
        this.diagnosticCollection.delete(document.uri);
    }

    dispose(): void {
        this.diagnosticCollection.dispose();
    }
}