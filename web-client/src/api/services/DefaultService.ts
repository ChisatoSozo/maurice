/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { GetLoadedModelNameArgs } from '../models/GetLoadedModelNameArgs';
import type { GetLoadedModelNameReturn } from '../models/GetLoadedModelNameReturn';
import type { ListChatModelsArgs } from '../models/ListChatModelsArgs';
import type { ListChatModelsReturn } from '../models/ListChatModelsReturn';
import type { LoadModelArgs } from '../models/LoadModelArgs';
import type { LoadModelReturn } from '../models/LoadModelReturn';
import type { SendChatArgs } from '../models/SendChatArgs';
import type { SendChatReturn } from '../models/SendChatReturn';
import type { CancelablePromise } from '../core/CancelablePromise';
import { OpenAPI } from '../core/OpenAPI';
import { request as __request } from '../core/request';
export class DefaultService {
    /**
     * @param body
     * @returns GetLoadedModelNameReturn OK
     * @throws ApiError
     */
    public static postApiGetLoadedModelName(
        body: GetLoadedModelNameArgs,
    ): CancelablePromise<GetLoadedModelNameReturn> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/api/get_loaded_model_name',
            body: body,
        });
    }
    /**
     * @param body
     * @returns ListChatModelsReturn OK
     * @throws ApiError
     */
    public static postApiListChatModels(
        body: ListChatModelsArgs,
    ): CancelablePromise<ListChatModelsReturn> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/api/list_chat_models',
            body: body,
        });
    }
    /**
     * @param body
     * @returns LoadModelReturn OK
     * @throws ApiError
     */
    public static postApiLoadModel(
        body: LoadModelArgs,
    ): CancelablePromise<LoadModelReturn> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/api/load_model',
            body: body,
        });
    }
    /**
     * @param body
     * @returns SendChatReturn OK
     * @throws ApiError
     */
    public static postApiSendChat(
        body: SendChatArgs,
    ): CancelablePromise<SendChatReturn> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/api/send_chat',
            body: body,
        });
    }
}
