import React from "react";
import { Box, IconButton, Typography } from "@mui/material";
import {
  Add,
  ChevronRight,
  CreateNewFolder,
  Delete,
} from "@mui/icons-material";
import { MauriceApi } from "../api";

const getLastBitFromPath = (path: string) => {
  const parts = path.split("/");
  return parts[parts.length - 1];
};

export const DirectoryItem: React.FC<{
  directory: { name: string; open: boolean };
  onOpen: () => void;
  onCreateFile: () => void;
  onCreateDirectory: () => void;
  updateFolder: () => void;
  children: React.ReactNode;
}> = ({
  directory,
  onOpen,
  onCreateFile,
  onCreateDirectory,
  updateFolder,
  children,
}) => {
  const [isLoading, setIsLoading] = React.useState(false);
  if (isLoading) {
    return <Typography>Loading...</Typography>;
  }
  return (
    <>
      <Box display={"flex"} alignItems={"center"}>
        <IconButton onClick={onOpen}>
          <ChevronRight
            style={{
              transform: directory.open ? "rotate(90deg)" : "rotate(0deg)",
              transition: "transform 0.3s",
            }}
          />
        </IconButton>
        <Typography>{getLastBitFromPath(directory.name)}</Typography>
        <IconButton onClick={onCreateFile}>
          <Add />
        </IconButton>
        <IconButton onClick={onCreateDirectory}>
          <CreateNewFolder />
        </IconButton>
        <IconButton
          onClick={() => {
            setIsLoading(true);
            const pathStr = directory.name.replace("./files", "");
            MauriceApi.postApiDeleteFolder({ path: pathStr }).then(() => {
              updateFolder();
            });
          }}
        >
          <Delete />
        </IconButton>
      </Box>
      {children}
    </>
  );
};
