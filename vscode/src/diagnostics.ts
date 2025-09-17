// src/diagnostics.ts
import * as vscode from 'vscode';
import { QbitParser, ParseError } from './parser';

export class QbitDiagnosticsProvider {
    private diagnosticCollection: vscode.DiagnosticCollection;

    constructor() {
        this.diagnosticCollection = vscode.languages.createDiagnosticCollection('qbit');
    }

    async updateDiagnostics(document: vscode.TextDocument): Promise<void> {
        if (document.languageId !== 'qbit') {
            return;
        }

        try {
            const errors = await QbitParser.validateSyntax(document.getText());
            const diagnostics: vscode.Diagnostic[] = errors.map(error => {
                const range = new vscode.Range(
                    new vscode.Position(Math.max(0, error.line - 1), Math.max(0, error.column - 1)),
                    new vscode.Position(Math.max(0, error.line - 1), Math.max(0, error.column - 1 + error.length))
                );

                const diagnostic = new vscode.Diagnostic(
                    range,
                    error.message,
                    vscode.DiagnosticSeverity.Error
                );

                diagnostic.source = 'qbit';
                return diagnostic;
            });

            this.diagnosticCollection.set(document.uri, diagnostics);
        } catch (err) {
            console.error('Error updating diagnostics:', err);
            // Clear diagnostics on error
            this.diagnosticCollection.set(document.uri, []);
        }
    }

    clearDiagnostics(document: vscode.TextDocument): void {
        this.diagnosticCollection.delete(document.uri);
    }

    dispose(): void {
        this.diagnosticCollection.dispose();
    }
}