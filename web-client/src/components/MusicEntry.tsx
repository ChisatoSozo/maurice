import { Box, Button, Typography } from "@mui/material";
import { GetYoutubeVideosReturn } from "../api";
import { analyzeImageColors, ColorAnalysis } from "../utils/util";
import { useEffect, useState } from "react";
import AddIcon from "@mui/icons-material/Add";
import PlayArrowIcon from "@mui/icons-material/PlayArrow";

export const MusicEntry = ({
  video,
  hideControls,
  onPlay,
  onAdd,
}: {
  video: GetYoutubeVideosReturn["videos"][0];
  hideControls?: boolean;
  onPlay?: () => void;
  onAdd?: () => void;
}) => {
  const [color, setColor] = useState<ColorAnalysis | null>(null);

  useEffect(() => {
    analyzeImageColors(video.thumbnail).then(setColor);
  }, [video.thumbnail]);

  return (
    <Box
      style={{
        display: "flex",
        flexDirection: "row",
        justifyContent: "space-between",
        alignItems: "center",
        height: 100,
        borderRadius: 10,
        overflow: "hidden",
        backgroundColor: color?.dominant.rgb || "white",
      }}
    >
      <img
        src={video.thumbnail}
        width={178}
        height={100}
        style={{
          objectFit: "cover",
        }}
      />
      <Box flex={1} height="100%">
        <Typography
          padding={1}
          color={color?.suggestedTextColor.rgb || "black"}
        >
          {video.title}
        </Typography>
      </Box>
      {!hideControls && (
        <>
          <Box>
            <Button
              style={{
                height: 60,
                width: 60,
                color: color?.suggestedTextColor.rgb || "black",
              }}
              onClick={onPlay}
            >
              <PlayArrowIcon
                style={{
                  fontSize: 40,
                  marginTop: 23,
                  marginLeft: 24,
                }}
              />
            </Button>
          </Box>
          <Box
            style={{
              marginRight: 10,
            }}
          >
            <Button
              style={{
                height: 60,
                width: 60,
                color: color?.suggestedTextColor.rgb || "black",
              }}
              onClick={onAdd}
            >
              <AddIcon
                style={{
                  fontSize: 40,
                  marginTop: 23,
                  marginLeft: 24,
                }}
              />
            </Button>
          </Box>
        </>
      )}
    </Box>
  );
};
