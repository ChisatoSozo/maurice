/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { GetLoadedModelNameArgs } from '../models/GetLoadedModelNameArgs';
import type { GetLoadedModelNameReturn } from '../models/GetLoadedModelNameReturn';
import type { GetSpeakersReturn } from '../models/GetSpeakersReturn';
import type { GetVolumeArgs } from '../models/GetVolumeArgs';
import type { GetVolumeReturn } from '../models/GetVolumeReturn';
import type { GetYoutubeVideosArgs } from '../models/GetYoutubeVideosArgs';
import type { GetYoutubeVideosReturn } from '../models/GetYoutubeVideosReturn';
import type { ListChatModelsArgs } from '../models/ListChatModelsArgs';
import type { ListChatModelsReturn } from '../models/ListChatModelsReturn';
import type { LoadModelArgs } from '../models/LoadModelArgs';
import type { LoadModelReturn } from '../models/LoadModelReturn';
import type { PlayAudioArgs } from '../models/PlayAudioArgs';
import type { SendChatArgs } from '../models/SendChatArgs';
import type { SendChatReturn } from '../models/SendChatReturn';
import type { SetVolumeArgs } from '../models/SetVolumeArgs';
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
     * @returns GetSpeakersReturn OK
     * @throws ApiError
     */
    public static postApiGetSpeakers(): CancelablePromise<GetSpeakersReturn> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/api/get_speakers',
        });
    }
    /**
     * @param body
     * @returns GetVolumeReturn OK
     * @throws ApiError
     */
    public static postApiGetVolume(
        body: GetVolumeArgs,
    ): CancelablePromise<GetVolumeReturn> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/api/get_volume',
            body: body,
        });
    }
    /**
     * @param body
     * @returns GetYoutubeVideosReturn OK
     * @throws ApiError
     */
    public static postApiGetYoutubeVideos(
        body: GetYoutubeVideosArgs,
    ): CancelablePromise<GetYoutubeVideosReturn> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/api/get_youtube_videos',
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
     * @returns boolean OK
     * @throws ApiError
     */
    public static postApiPlayAudio(
        body: PlayAudioArgs,
    ): CancelablePromise<boolean> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/api/play_audio',
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
    /**
     * @param body
     * @returns boolean OK
     * @throws ApiError
     */
    public static postApiSetVolume(
        body: SetVolumeArgs,
    ): CancelablePromise<boolean> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/api/set_volume',
            body: body,
        });
    }
}
