// src/parser.ts
import * as wasm from '../pkg/qbit_lang';

export interface ParseError {
    message: string;
    line: number;
    column: number;
    length: number;
}

export interface ParseResult {
    success: boolean;
    ast?: string;
    errors: ParseError[];
}

export class QbitParser {
    private static initialized = false;

    static async init() {
        if (!this.initialized) {
            // await wasm.default(); // Initialize WASM module
            wasm.init_panic_hook();
            this.initialized = true;
        }
    }

    static async parse(source: string): Promise<ParseResult> {
        await this.init();
        return wasm.parse_code(source);
    }

    static async validateSyntax(source: string): Promise<ParseError[]> {
        await this.init();
        return wasm.parse_syntax(source);
    }

    // Helper method to get tokens for syntax highlighting
    static getTokens(source: string): Array<{type: string, start: number, end: number}> {
        // For now, return empty array - can implement tokenization later
        return [];
    }

    // Helper method to find symbol at position
    static getSymbolAtPosition(source: string, line: number, character: number): string | null {
        const lines = source.split('\n');
        if (line >= lines.length) return null;
        
        const currentLine = lines[line];
        if (character >= currentLine.length) return null;

        // Simple word boundary detection
        const wordRegex = /[a-zA-Z_][a-zA-Z0-9_]*/g;
        let match;
        while ((match = wordRegex.exec(currentLine)) !== null) {
            if (character >= match.index && character < match.index + match[0].length) {
                return match[0];
            }
        }
        
        return null;
    }

    // Helper method to find function calls for completion
    static getFunctionCalls(source: string): string[] {
        const functionRegex = /([a-zA-Z_][a-zA-Z0-9_]*)\s*\(/g;
        const functions = new Set<string>();
        let match;

        while ((match = functionRegex.exec(source)) !== null) {
            functions.add(match[1]);
        }

        return Array.from(functions);
    }

    // Helper method to find variable declarations
    static getVariableDeclarations(source: string): Array<{name: string, type: 'let' | 'const', line: number}> {
        const varRegex = /(let|const)\s+([a-zA-Z_][a-zA-Z0-9_]*)/g;
        const variables: Array<{name: string, type: 'let' | 'const', line: number}> = [];
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

    // Helper method to find function definitions
    static getFunctionDefinitions(source: string): Array<{name: string, params: string[], line: number}> {
        const fnRegex = /fn\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\(([^)]*)\)/g;
        const functions: Array<{name: string, params: string[], line: number}> = [];
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