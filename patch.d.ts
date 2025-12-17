export declare interface NodeRepr {
  select(selectors: "html"): NodeRepr;
  select(selectors: "head"): NodeRepr;
  select(selectors: "body"): NodeRepr;
}

export declare class DOMParser {
  parseFromString(string: string, mimeType: string): NodeRepr;
}

