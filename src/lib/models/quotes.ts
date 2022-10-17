export namespace IQuotes {
    export interface RandomQuote {
        _id: string;
        content: string;
        author: string;
        tags: string[];
        authorSlug: string;
        length: number;
        dateAdded: string;
        dateModified: string;
    }

    export interface Tag {
        _id: string;
        name: string;
        slug: string;
        quoteCount: number;
        dateAdded: string;
        dateModified: string;
    }
}
