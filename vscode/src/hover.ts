// src/hover.ts
import * as vscode from 'vscode';
import { QbitParser } from './parser';

export class QbitHoverProvider implements vscode.HoverProvider {
    
    async provideHover(
        document: vscode.TextDocument,
        position: vscode.Position,
        token: vscode.CancellationToken
    ): Promise<vscode.Hover | undefined> {
        const source = document.getText();
        const line = position.line;
        const character = position.character;

        // Get the symbol at the current position
        const symbol = QbitParser.getSymbolAtPosition(source, line, character);
        if (!symbol) {
            return undefined;
        }

        // Check if it's a function definition
        const functions = QbitParser.getFunctionDefinitions(source);
        const functionDef = functions.find(f => f.name === symbol);
        if (functionDef) {
            const paramsStr = functionDef.params.length > 0 
                ? functionDef.params.join(', ')
                : '';
            
            const markdown = new vscode.MarkdownString();
            markdown.appendCodeblock(`fn ${symbol}(${paramsStr})`, 'qbit');
            markdown.appendMarkdown(`\n\nFunction defined at line ${functionDef.line + 1}`);
            
            return new vscode.Hover(markdown);
        }

        // Check if it's a variable declaration
        const variables = QbitParser.getVariableDeclarations(source);
        const variableDef = variables.find(v => v.name === symbol);
        if (variableDef) {
            const markdown = new vscode.MarkdownString();
            markdown.appendCodeblock(`${variableDef.type} ${symbol}`, 'qbit');
            markdown.appendMarkdown(`\n\nVariable declared at line ${variableDef.line + 1}`);
            
            return new vscode.Hover(markdown);
        }

        // Check if it's a built-in keyword
        const keywords = [
            'let', 'const', 'fn', 'if', 'else', 'while', 'for', 'return', 
            'break', 'continue', 'import', 'export', 'true', 'false', 'null'
        ];
        
        if (keywords.includes(symbol)) {
            const descriptions: { [key: string]: string } = {
                'let': 'Declares a mutable variable',
                'const': 'Declares an immutable constant',
                'fn': 'Declares a function',
                'if': 'Conditional statement',
                'else': 'Alternative branch for if statement',
                'while': 'Loop that continues while condition is true',
                'for': 'C-style for loop',
                'return': 'Returns a value from a function',
                'break': 'Exits the current loop',
                'continue': 'Skips to the next iteration of a loop',
                'import': 'Imports a module',
                'export': 'Exports a declaration',
                'true': 'Boolean true value',
                'false': 'Boolean false value',
                'null': 'Null value'
            };

            const description = descriptions[symbol] || 'Qbit keyword';
            const markdown = new vscode.MarkdownString();
            markdown.appendCodeblock(symbol, 'qbit');
            markdown.appendMarkdown(`\n\n${description}`);
            
            return new vscode.Hover(markdown);
        }

        // Default hover for unknown symbols
        const markdown = new vscode.MarkdownString();
        markdown.appendText(`Symbol: ${symbol}`);
        return new vscode.Hover(markdown);
    }
}