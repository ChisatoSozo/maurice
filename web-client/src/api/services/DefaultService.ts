/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { AppendSongToPlaylistArgs } from '../models/AppendSongToPlaylistArgs';
import type { CompleteArgs } from '../models/CompleteArgs';
import type { CompleteReturn } from '../models/CompleteReturn';
import type { CreateDirectoryArgs } from '../models/CreateDirectoryArgs';
import type { CreateFileArgs } from '../models/CreateFileArgs';
import type { DeleteFileArgs } from '../models/DeleteFileArgs';
import type { DeleteFolderArgs } from '../models/DeleteFolderArgs';
import type { EditFileArgs } from '../models/EditFileArgs';
import type { GetLoadedChatModelNameArgs } from '../models/GetLoadedChatModelNameArgs';
import type { GetLoadedChatModelNameReturn } from '../models/GetLoadedChatModelNameReturn';
import type { GetLoadedCompletionModelNameArgs } from '../models/GetLoadedCompletionModelNameArgs';
import type { GetLoadedCompletionModelNameReturn } from '../models/GetLoadedCompletionModelNameReturn';
import type { GetPlaylistArgs } from '../models/GetPlaylistArgs';
import type { GetPlaylistReturn } from '../models/GetPlaylistReturn';
import type { GetSongDurationReturn } from '../models/GetSongDurationReturn';
import type { GetSongTimeArgs } from '../models/GetSongTimeArgs';
import type { GetSongTimeReturn } from '../models/GetSongTimeReturn';
import type { GetSpeakersReturn } from '../models/GetSpeakersReturn';
import type { GetVolumeArgs } from '../models/GetVolumeArgs';
import type { GetVolumeReturn } from '../models/GetVolumeReturn';
import type { GetYoutubeVideosArgs } from '../models/GetYoutubeVideosArgs';
import type { GetYoutubeVideosReturn } from '../models/GetYoutubeVideosReturn';
import type { IsLockedArgs } from '../models/IsLockedArgs';
import type { IsLockedResponse } from '../models/IsLockedResponse';
import type { ListChatModelsArgs } from '../models/ListChatModelsArgs';
import type { ListChatModelsReturn } from '../models/ListChatModelsReturn';
import type { ListCompletionModelsArgs } from '../models/ListCompletionModelsArgs';
import type { ListCompletionModelsReturn } from '../models/ListCompletionModelsReturn';
import type { ListFilesArgs } from '../models/ListFilesArgs';
import type { ListFilesResponse } from '../models/ListFilesResponse';
import type { LoadChatModelArgs } from '../models/LoadChatModelArgs';
import type { LoadChatModelReturn } from '../models/LoadChatModelReturn';
import type { LoadCompletionModelArgs } from '../models/LoadCompletionModelArgs';
import type { LoadCompletionModelReturn } from '../models/LoadCompletionModelReturn';
import type { PauseArgs } from '../models/PauseArgs';
import type { PlayAudioArgs } from '../models/PlayAudioArgs';
import type { RemoveSongFromPlaylistAtIndexArgs } from '../models/RemoveSongFromPlaylistAtIndexArgs';
import type { ResumeArgs } from '../models/ResumeArgs';
import type { SendChatArgs } from '../models/SendChatArgs';
import type { SendChatReturn } from '../models/SendChatReturn';
import type { SetSongTimeArgs } from '../models/SetSongTimeArgs';
import type { SetVolumeArgs } from '../models/SetVolumeArgs';
import type { StopArgs } from '../models/StopArgs';
import type { CancelablePromise } from '../core/CancelablePromise';
import { OpenAPI } from '../core/OpenAPI';
import { request as __request } from '../core/request';
export class DefaultService {
    /**
     * @param body
     * @returns boolean OK
     * @throws ApiError
     */
    public static postApiAppendSongToPlaylist(
        body: AppendSongToPlaylistArgs,
    ): CancelablePromise<boolean> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/api/append_song_to_playlist',
            body: body,
        });
    }
    /**
     * @param body
     * @returns CompleteReturn OK
     * @throws ApiError
     */
    public static postApiComplete(
        body: CompleteArgs,
    ): CancelablePromise<CompleteReturn> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/api/complete',
            body: body,
        });
    }
    /**
     * @param body
     * @returns boolean OK
     * @throws ApiError
     */
    public static postApiCreateDirectory(
        body: CreateDirectoryArgs,
    ): CancelablePromise<boolean> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/api/create_directory',
            body: body,
        });
    }
    /**
     * @param body
     * @returns boolean OK
     * @throws ApiError
     */
    public static postApiCreateFile(
        body: CreateFileArgs,
    ): CancelablePromise<boolean> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/api/create_file',
            body: body,
        });
    }
    /**
     * @param body
     * @returns boolean OK
     * @throws ApiError
     */
    public static postApiDeleteFile(
        body: DeleteFileArgs,
    ): CancelablePromise<boolean> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/api/delete_file',
            body: body,
        });
    }
    /**
     * @param body
     * @returns boolean OK
     * @throws ApiError
     */
    public static postApiDeleteFolder(
        body: DeleteFolderArgs,
    ): CancelablePromise<boolean> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/api/delete_folder',
            body: body,
        });
    }
    /**
     * @param body
     * @returns boolean OK
     * @throws ApiError
     */
    public static postApiEditFile(
        body: EditFileArgs,
    ): CancelablePromise<boolean> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/api/edit_file',
            body: body,
        });
    }
    /**
     * @param body
     * @returns GetLoadedChatModelNameReturn OK
     * @throws ApiError
     */
    public static postApiGetLoadedChatModelName(
        body: GetLoadedChatModelNameArgs,
    ): CancelablePromise<GetLoadedChatModelNameReturn> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/api/get_loaded_chat_model_name',
            body: body,
        });
    }
    /**
     * @param body
     * @returns GetLoadedCompletionModelNameReturn OK
     * @throws ApiError
     */
    public static postApiGetLoadedCompletionModelName(
        body: GetLoadedCompletionModelNameArgs,
    ): CancelablePromise<GetLoadedCompletionModelNameReturn> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/api/get_loaded_completion_model_name',
            body: body,
        });
    }
    /**
     * @param body
     * @returns GetPlaylistReturn OK
     * @throws ApiError
     */
    public static postApiGetPlaylist(
        body: GetPlaylistArgs,
    ): CancelablePromise<GetPlaylistReturn> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/api/get_playlist',
            body: body,
        });
    }
    /**
     * @param body
     * @returns GetSongDurationReturn OK
     * @throws ApiError
     */
    public static postApiGetSongDuration(
        body: GetSongTimeArgs,
    ): CancelablePromise<GetSongDurationReturn> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/api/get_song_duration',
            body: body,
        });
    }
    /**
     * @param body
     * @returns GetSongTimeReturn OK
     * @throws ApiError
     */
    public static postApiGetSongTime(
        body: GetSongTimeArgs,
    ): CancelablePromise<GetSongTimeReturn> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/api/get_song_time',
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
     * @returns IsLockedResponse OK
     * @throws ApiError
     */
    public static postApiIsLocked(
        body: IsLockedArgs,
    ): CancelablePromise<IsLockedResponse> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/api/is_locked',
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
     * @returns ListCompletionModelsReturn OK
     * @throws ApiError
     */
    public static postApiListCompletionModels(
        body: ListCompletionModelsArgs,
    ): CancelablePromise<ListCompletionModelsReturn> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/api/list_completion_models',
            body: body,
        });
    }
    /**
     * @param body
     * @returns ListFilesResponse OK
     * @throws ApiError
     */
    public static postApiListFiles(
        body: ListFilesArgs,
    ): CancelablePromise<ListFilesResponse> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/api/list_files',
            body: body,
        });
    }
    /**
     * @param body
     * @returns LoadChatModelReturn OK
     * @throws ApiError
     */
    public static postApiLoadChatModel(
        body: LoadChatModelArgs,
    ): CancelablePromise<LoadChatModelReturn> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/api/load_chat_model',
            body: body,
        });
    }
    /**
     * @param body
     * @returns LoadCompletionModelReturn OK
     * @throws ApiError
     */
    public static postApiLoadCompletionModel(
        body: LoadCompletionModelArgs,
    ): CancelablePromise<LoadCompletionModelReturn> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/api/load_completion_model',
            body: body,
        });
    }
    /**
     * @param body
     * @returns boolean OK
     * @throws ApiError
     */
    public static postApiPause(
        body: PauseArgs,
    ): CancelablePromise<boolean> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/api/pause',
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
     * @returns boolean OK
     * @throws ApiError
     */
    public static postApiRemoveSongFromPlaylistAtIndex(
        body: RemoveSongFromPlaylistAtIndexArgs,
    ): CancelablePromise<boolean> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/api/remove_song_from_playlist_at_index',
            body: body,
        });
    }
    /**
     * @param body
     * @returns boolean OK
     * @throws ApiError
     */
    public static postApiResume(
        body: ResumeArgs,
    ): CancelablePromise<boolean> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/api/resume',
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
    public static postApiSetSongTime(
        body: SetSongTimeArgs,
    ): CancelablePromise<boolean> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/api/set_song_time',
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
    /**
     * @param body
     * @returns boolean OK
     * @throws ApiError
     */
    public static postApiStop(
        body: StopArgs,
    ): CancelablePromise<boolean> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/api/stop',
            body: body,
        });
    }
}
