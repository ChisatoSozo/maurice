import React, { useEffect, useState } from "react";
import { Box, IconButton, Typography } from "@mui/material";
import { Delete } from "@mui/icons-material";
import { Link } from "react-router-dom";
import { MauriceApi } from "../api";

const getLastBitFromPath = (path: string) => {
  const parts = path.split("/");
  return parts[parts.length - 1];
};

export const FileItem: React.FC<{ path: string; updateFolder: () => void }> = ({
  path,
  updateFolder,
}) => {
  const [isLoading, setIsLoading] = useState(true);
  const [isLocked, setIsLocked] = useState(false);
  const [isMine, setIsMine] = useState(false);

  useEffect(() => {
    if (path.endsWith(".lock")) {
      return;
    }
    const pathStr = path.replace("./files", "");
    MauriceApi.postApiIsLocked({ path: pathStr }).then((response) => {
      setIsLocked(response.locked);
      setIsMine(response.locked_by_me);
      setIsLoading(false);
    });
  }, [path]);

  if (path.endsWith(".lock")) {
    return null;
  }

  if (isLoading) {
    return <Typography>Loading...</Typography>;
  }

  if (isLocked && !isMine) {
    return (
      <Typography>
        {getLastBitFromPath(path)} - Locked by another user
      </Typography>
    );
  }

  const pathStr = path.replace("./files", "");

  return (
    <Box display={"flex"} alignItems={"center"}>
      <Link
        to={`/edit${pathStr}`}
        style={{
          padding: 0,
          textTransform: "none",
        }}
      >
        <Typography>
          {getLastBitFromPath(path)} {isLocked && isMine && " - Locked by you"}
        </Typography>
      </Link>
      <IconButton
        onClick={() => {
          setIsLoading(true);
          MauriceApi.postApiDeleteFile({ path: pathStr }).then(() => {
            updateFolder();
          });
        }}
      >
        <Delete />
      </IconButton>
    </Box>
  );
};
