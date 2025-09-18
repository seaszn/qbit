import { parse_code, init_panic } from '../pkg/qbit_lang';
import * as vscode from 'vscode';

// export interface ParseMessage {
//     message: string;
//     line: number;
//     column: number;
//     length: number;
// }

export interface Diagnostic {
    level: vscode.DiagnosticSeverity
    message: string;
    line: number;
    column: number;
    length: number;
}

export interface ParseResult {
    success: boolean;
    diagnositcs: Diagnostic[];
}

export class Parser {
    private static initialized = false;

    static async init() {
        init_panic();
    }

    static async parse(source: string): Promise<ParseResult> {
        await this.init();

        return parse_code(source);
    }

    static getSymbolAtPosition(source: string, line: number, character: number): string | null {
        const lines = source.split('\n');
        if (line >= lines.length) return null;

        const currentLine = lines[line];
        if (character >= currentLine.length) return null;

        const wordRegex = /[a-zA-Z_][a-zA-Z0-9_]*/g;
        let match;
        while ((match = wordRegex.exec(currentLine)) !== null) {
            if (character >= match.index && character < match.index + match[0].length) {
                return match[0];
            }
        }

        return null;
    }

    static getVariableDeclarations(source: string): Array<{ name: string, type: 'let' | 'const', line: number }> {
        const varRegex = /(let|const)\s+([a-zA-Z_][a-zA-Z0-9_]*)/g;
        const variables: Array<{ name: string, type: 'let' | 'const', line: number }> = [];
        const lines = source.split('\n');

        lines.forEach((line, index) => {
            let match;
            const lineRegex = /(let|const)\s+([a-zA-Z_][a-zA-Z0-9_]*)/g;
            while ((match = lineRegex.exec(line)) !== null) {
                variables.push({
                    name: match[2],
                    type: match[1] as 'let' | 'const',
                    line: index
                });
            }
        });

        return variables;
    }

    static getFunctionDefinitions(source: string): Array<{ name: string, params: string[], line: number }> {
        const fnRegex = /fn\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\(([^)]*)\)/g;
        const functions: Array<{ name: string, params: string[], line: number }> = [];
        const lines = source.split('\n');

        lines.forEach((line, index) => {
            let match;
            const lineRegex = /fn\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\(([^)]*)\)/g;
            while ((match = lineRegex.exec(line)) !== null) {
                const params = match[2]
                    .split(',')
                    .map(p => p.trim())
                    .filter(p => p.length > 0);

                functions.push({
                    name: match[1],
                    params,
                    line: index
                });
            }
        });

        return functions;
    }
}