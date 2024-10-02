/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
/**
 * LoadChatModelArgs
 *
 * <details><summary>JSON schema</summary>
 *
 * ```json
 * {
     * "type": "object",
     * "required": [
         * "cache_quant",
         * "model_name"
         * ],
         * "properties": {
             * "cache_quant": {
                 * "type": "integer"
                 * },
                 * "model_name": {
                     * "type": "string"
                     * }
                     * }
                     * }
                     * ```
                     * </details>
                     */
                    export type LoadChatModelArgs = {
                        cache_quant: number;
                        model_name: string;
                    };

