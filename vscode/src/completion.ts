// src/completion.ts
import * as vscode from 'vscode';
import { QbitParser } from './parser';

export class QbitCompletionProvider implements vscode.CompletionItemProvider {
    async provideCompletionItems(
        document: vscode.TextDocument,
        position: vscode.Position,
        token: vscode.CancellationToken,
        context: vscode.CompletionContext
    ): Promise<vscode.CompletionItem[] | vscode.CompletionList> {
        const source = document.getText();
        const completionItems: vscode.CompletionItem[] = [];

        // Add keyword completions
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

        // Add variable completions
        const variables = QbitParser.getVariableDeclarations(source);
        variables.forEach(variable => {
            const item = new vscode.CompletionItem(variable.name, vscode.CompletionItemKind.Variable);
            item.detail = `${variable.type} ${variable.name}`;
            completionItems.push(item);
        });

        // Add snippet completions
        const snippets = this.getSnippets();
        completionItems.push(...snippets);

        return completionItems;
    }

    private getSnippets(): vscode.CompletionItem[] {
        const snippets: vscode.CompletionItem[] = [];

        // Function declaration snippet
        const fnSnippet = new vscode.CompletionItem('fn', vscode.CompletionItemKind.Snippet);
        fnSnippet.insertText = new vscode.SnippetString('fn ${1:name}(${2:params}) {\n\t${3:// body}\n}');
        fnSnippet.detail = 'Function declaration';
        fnSnippet.documentation = new vscode.MarkdownString('Create a new function');
        snippets.push(fnSnippet);

        // If statement snippet
        const ifSnippet = new vscode.CompletionItem('if', vscode.CompletionItemKind.Snippet);
        ifSnippet.insertText = new vscode.SnippetString('if ${1:condition} {\n\t${2:// body}\n}');
        ifSnippet.detail = 'If statement';
        ifSnippet.documentation = new vscode.MarkdownString('Create an if statement');
        snippets.push(ifSnippet);

        // If-else statement snippet
        const ifElseSnippet = new vscode.CompletionItem('ifelse', vscode.CompletionItemKind.Snippet);
        ifElseSnippet.insertText = new vscode.SnippetString('if ${1:condition} {\n\t${2:// if body}\n} else {\n\t${3:// else body}\n}');
        ifElseSnippet.detail = 'If-else statement';
        ifElseSnippet.documentation = new vscode.MarkdownString('Create an if-else statement');
        snippets.push(ifElseSnippet);

        // While loop snippet
        const whileSnippet = new vscode.CompletionItem('while', vscode.CompletionItemKind.Snippet);
        whileSnippet.insertText = new vscode.SnippetString('while ${1:condition} {\n\t${2:// body}\n}');
        whileSnippet.detail = 'While loop';
        whileSnippet.documentation = new vscode.MarkdownString('Create a while loop');
        snippets.push(whileSnippet);

        // For loop snippet
        const forSnippet = new vscode.CompletionItem('for', vscode.CompletionItemKind.Snippet);
        forSnippet.insertText = new vscode.SnippetString('for (${1:let i = 0}; ${2:i < 10}; ${3:i++}) {\n\t${4:// body}\n}');
        forSnippet.detail = 'For loop';
        forSnippet.documentation = new vscode.MarkdownString('Create a for loop');
        snippets.push(forSnippet);

        return snippets;
    }
} 
// Items.push(item);
//         });

// // Add function completions
// const functions = QbitParser.getFunctionDefinitions(source);
// functions.forEach(func => {
//     const item = new vscode.CompletionItem(func.name, vscode.CompletionItemKind.Function);
//     item.detail = `fn ${func.name}(${func.params.join(', ')})`;

//     // Create snippet for function call
//     const params = func.params.map((param, index) => `\${${index + 1}:${param}}`).join(', ');
//     item.insertText = new vscode.SnippetString(`${func.name}(${params})`);

//     completion