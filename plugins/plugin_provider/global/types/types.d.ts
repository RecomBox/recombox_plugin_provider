declare module 'fast-text-encoding' {
    export class TextDecoder {
        constructor(label?: string, options?: { fatal?: boolean; ignoreBOM?: boolean });
        decode(input?: Uint8Array): string;
    }

    export class TextEncoder {
        encode(input?: string): Uint8Array;
    }
}
