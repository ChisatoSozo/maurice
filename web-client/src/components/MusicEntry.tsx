import { Box, Typography, useMediaQuery } from "@mui/material";
import { GetYoutubeVideosReturn } from "../api";
import { analyzeImageColors, ColorAnalysis } from "../utils/util";
import { useEffect, useState } from "react";
import { SongTime } from "./SongTime";

export const MusicEntry = ({
  video,
  controls,
  speaker,
  showTime,
}: {
  video: GetYoutubeVideosReturn["videos"][0];
  controls: (textColor: string) => React.ReactNode;
  speaker: string;
  showTime?: boolean;
}) => {
  const [color, setColor] = useState<ColorAnalysis | null>(null);

  useEffect(() => {
    analyzeImageColors(video.thumbnail).then(setColor);
  }, [video.thumbnail]);

  const small = useMediaQuery("(max-width: 600px)");

  if (small) {
    return (
      <Box
        display={"flex"}
        flexDirection={"column"}
        style={{
          height: showTime ? 220 : 160,
          borderRadius: 10,
          overflow: "hidden",
          backgroundColor: color?.dominant.rgb || "white",
        }}
      >
        <Box
          style={{
            display: "flex",
            flex: 1,
            flexDirection: "row",
            justifyContent: "space-between",
            alignItems: "center",
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
              height={80}
            >
              {video.title}
            </Typography>
          </Box>
        </Box>
        <Box
          style={{
            display: "flex",
            flexDirection: "row",
            justifyContent: "space-between",
            alignItems: "center",
            width: 178,
          }}
        >
          {controls(color?.suggestedTextColor.rgb || "black")}
        </Box>
        {showTime && (
          <SongTime
            speaker={speaker}
            song={video}
            color={color?.suggestedTextColor.rgb || "black"}
          />
        )}
      </Box>
    );
  }

  return (
    <Box
      display={"flex"}
      flexDirection={"column"}
      style={{
        height: showTime ? 160 : 100,
        borderRadius: 10,
        overflow: "hidden",
        backgroundColor: color?.dominant.rgb || "white",
      }}
    >
      <Box
        style={{
          display: "flex",
          flex: 1,
          flexDirection: "row",
          justifyContent: "space-between",
          alignItems: "center",
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

        {controls(color?.suggestedTextColor.rgb || "black")}
      </Box>
      {showTime && (
        <SongTime
          speaker={speaker}
          song={video}
          color={color?.suggestedTextColor.rgb || "black"}
        />
      )}
    </Box>
  );
};
