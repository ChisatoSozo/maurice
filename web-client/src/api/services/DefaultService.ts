/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { SayHelloInput } from '../models/SayHelloInput';
import type { SayHelloOutput } from '../models/SayHelloOutput';
import type { CancelablePromise } from '../core/CancelablePromise';
import { OpenAPI } from '../core/OpenAPI';
import { request as __request } from '../core/request';
export class DefaultService {
    /**
     * @param body
     * @returns SayHelloOutput OK
     * @throws ApiError
     */
    public static postApiSayHello(
        body: SayHelloInput,
    ): CancelablePromise<SayHelloOutput> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/api/say_hello',
            body: body,
        });
    }
}
