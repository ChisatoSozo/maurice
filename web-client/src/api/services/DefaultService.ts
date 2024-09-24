/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { AppendSongToPlaylistArgs } from '../models/AppendSongToPlaylistArgs';
import type { CreateDirectoryArgs } from '../models/CreateDirectoryArgs';
import type { CreateFileArgs } from '../models/CreateFileArgs';
import type { DeleteFileArgs } from '../models/DeleteFileArgs';
import type { DeleteFolderArgs } from '../models/DeleteFolderArgs';
import type { EditFileArgs } from '../models/EditFileArgs';
import type { GetLoadedModelNameArgs } from '../models/GetLoadedModelNameArgs';
import type { GetLoadedModelNameReturn } from '../models/GetLoadedModelNameReturn';
import type { GetPlaylistArgs } from '../models/GetPlaylistArgs';
import type { GetPlaylistReturn } from '../models/GetPlaylistReturn';
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
import type { ListFilesAndDirectoriesArgs } from '../models/ListFilesAndDirectoriesArgs';
import type { ListFilesAndDirectoriesResponse } from '../models/ListFilesAndDirectoriesResponse';
import type { LoadModelArgs } from '../models/LoadModelArgs';
import type { LoadModelReturn } from '../models/LoadModelReturn';
import type { PauseArgs } from '../models/PauseArgs';
import type { PlayAudioArgs } from '../models/PlayAudioArgs';
import type { RemoveSongFromPlaylistAtIndexArgs } from '../models/RemoveSongFromPlaylistAtIndexArgs';
import type { ResumeArgs } from '../models/ResumeArgs';
import type { SendChatArgs } from '../models/SendChatArgs';
import type { SendChatReturn } from '../models/SendChatReturn';
import type { SetSongTimeArgs } from '../models/SetSongTimeArgs';
import type { SetVolumeArgs } from '../models/SetVolumeArgs';
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
     * @returns ListFilesAndDirectoriesResponse OK
     * @throws ApiError
     */
    public static postApiListFilesAndDirectories(
        body: ListFilesAndDirectoriesArgs,
    ): CancelablePromise<ListFilesAndDirectoriesResponse> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/api/list_files_and_directories',
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
}
