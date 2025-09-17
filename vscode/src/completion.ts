import * as vscode from 'vscode';
import { Parser } from './parser';

export class CompletionProvider implements vscode.CompletionItemProvider {
    
    async provideCompletionItems(
        document: vscode.TextDocument,
        position: vscode.Position,
        token: vscode.CancellationToken,
        context: vscode.CompletionContext
    ): Promise<vscode.CompletionItem[] | vscode.CompletionList> {
        const source = document.getText();
        const completionItems: vscode.CompletionItem[] = [];

        const keywords = [
            { name: 'let', kind: vscode.CompletionItemKind.Keyword, detail: 'Variable declaration' },
            { name: 'const', kind: vscode.CompletionItemKind.Keyword, detail: 'Constant declaration' },
            { name: 'fn', kind: vscode.CompletionItemKind.Keyword, detail: 'Function declaration' },
            { name: 'if', kind: vscode.CompletionItemKind.Keyword, detail: 'Conditional statement' },
            { name: 'else', kind: vscode.CompletionItemKind.Keyword, detail: 'Alternative branch' },
            { name: 'while', kind: vscode.CompletionItemKind.Keyword, detail: 'While loop' },
            { name: 'for', kind: vscode.CompletionItemKind.Keyword, detail: 'For loop' },
            { name: 'return', kind: vscode.CompletionItemKind.Keyword, detail: 'Return statement' },
            { name: 'break', kind: vscode.CompletionItemKind.Keyword, detail: 'Break statement' },
            { name: 'continue', kind: vscode.CompletionItemKind.Keyword, detail: 'Continue statement' },
            { name: 'import', kind: vscode.CompletionItemKind.Keyword, detail: 'Import statement' },
            { name: 'export', kind: vscode.CompletionItemKind.Keyword, detail: 'Export statement' },
            { name: 'true', kind: vscode.CompletionItemKind.Keyword, detail: 'Boolean true' },
            { name: 'false', kind: vscode.CompletionItemKind.Keyword, detail: 'Boolean false' },
            { name: 'null', kind: vscode.CompletionItemKind.Keyword, detail: 'Null value' }
        ];

        keywords.forEach(keyword => {
            const item = new vscode.CompletionItem(keyword.name, keyword.kind);
            item.detail = keyword.detail;
            completionItems.push(item);
        });

        const functions = Parser.getFunctionDefinitions(source);
        functions.forEach(func => {
            const item = new vscode.CompletionItem(func.name, vscode.CompletionItemKind.Function);
            item.detail = `fn ${func.name}(${func.params.join(', ')})`;
            
            const params = func.params.map((param, index) => `\${${index + 1}:${param}}`).join(', ');
            item.insertText = new vscode.SnippetString(`${func.name}(${params})`);
            
            completionItems.push(item);
        });

        const variables = Parser.getVariableDeclarations(source);
        variables.forEach(variable => {
            const item = new vscode.CompletionItem(variable.name, vscode.CompletionItemKind.Variable);
            item.detail = `${variable.type} ${variable.name}`;
            completionItems.push(item);
        });

        // Add snippets
        const fnSnippet = new vscode.CompletionItem('fn', vscode.CompletionItemKind.Snippet);
        fnSnippet.insertText = new vscode.SnippetString('fn ${1:name}(${2:params}) {\n\t${3:// body}\n}');
        fnSnippet.detail = 'Function declaration';
        completionItems.push(fnSnippet);

        const ifSnippet = new vscode.CompletionItem('if', vscode.CompletionItemKind.Snippet);
        ifSnippet.insertText = new vscode.SnippetString('if ${1:condition} {\n\t${2:// body}\n}');
        ifSnippet.detail = 'If statement';
        completionItems.push(ifSnippet);

        return completionItems;
    }
}