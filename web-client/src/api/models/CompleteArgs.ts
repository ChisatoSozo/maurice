/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
/**
 * CompleteArgs
 *
 * <details><summary>JSON schema</summary>
 *
 * ```json
 * {
     * "type": "object",
     * "required": [
         * "max_new_tokens",
         * "prompt"
         * ],
         * "properties": {
             * "max_new_tokens": {
                 * "type": "integer"
                 * },
                 * "prompt": {
                     * "type": "string"
                     * }
                     * }
                     * }
                     * ```
                     * </details>
                     */
                    export type CompleteArgs = {
                        max_new_tokens: number;
                        prompt: string;
                    };

