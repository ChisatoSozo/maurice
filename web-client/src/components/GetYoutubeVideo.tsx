import { Box, Button, Input } from "@mui/material";
import { useCallback, useState } from "react";
import { GetYoutubeVideosReturn, MauriceApi } from "../api";
import { MusicEntry } from "./MusicEntry";
import { Add, PlayArrow } from "@mui/icons-material";

export const GetYoutubeVideo = ({
  onAddVideo,
  onPlayVideo,
  speaker,
}: {
  onAddVideo?: (video: GetYoutubeVideosReturn["videos"][0]) => void;
  onPlayVideo?: (video: GetYoutubeVideosReturn["videos"][0]) => void;
  speaker: string;
}) => {
  const [searchValue, setSearchValue] = useState("");
  const [videos, setVideos] = useState<GetYoutubeVideosReturn["videos"]>([]);
  const [loading, setLoading] = useState(false);

  const search = useCallback(async () => {
    if (!searchValue) {
      return;
    }
    setLoading(true);
    const videos = await MauriceApi.postApiGetYoutubeVideos({
      search: searchValue,
    });
    setLoading(false);
    setVideos(videos.videos);
  }, [searchValue]);

  return (
    <Box display="flex" flexDirection="column" gap={1}>
      <Input
        placeholder="Enter a youtube video name"
        endAdornment={
          <Button disabled={!searchValue || loading} onClick={search}>
            {loading ? "Loading..." : "Search"}
          </Button>
        }
        onKeyDown={(e) => {
          if (e.key === "Enter") {
            search();
          }
        }}
        onChange={(e) => {
          setSearchValue(e.target.value);
        }}
        value={searchValue}
        style={{
          width: "100%",
        }}
      />
      {videos.map((video) => (
        <MusicEntry
          speaker={speaker}
          video={video}
          controls={(color) => (
            <>
              <Box
                style={{
                  marginRight: 10,
                }}
              >
                <Button
                  style={{
                    height: 60,
                    width: 60,
                    color: color,
                  }}
                  onClick={() => onAddVideo?.(video)}
                >
                  <Add />
                </Button>
              </Box>
              <Box>
                <Button
                  style={{
                    height: 60,
                    width: 60,
                    color,
                  }}
                  onClick={() => onPlayVideo?.(video)}
                >
                  <PlayArrow />
                </Button>
              </Box>
            </>
          )}
        />
      ))}
    </Box>
  );
};
