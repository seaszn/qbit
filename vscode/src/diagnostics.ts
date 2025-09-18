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
            const result = await Parser.parse(document.getText());
            
            console.log(result)

            const diagnostics: vscode.Diagnostic[] = result.diagnostics.map(x => {
                const range = new vscode.Range(
                    new vscode.Position(Math.max(0, x.line - 1), Math.max(0, x.column - 1)),
                    new vscode.Position(Math.max(0, x.line - 1), Math.max(0, x.column - 1 + x.length))
                );

                const diagnostic = new vscode.Diagnostic(
                    range,
                    x.message,
                    x.level
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