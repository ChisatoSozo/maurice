import React, { useCallback, useEffect, useState } from "react";
import { Box } from "@mui/material";
import { MauriceApi } from "../api";
import { FileItem } from "./FileItem";
import { DirectoryItem } from "./DirectoryItem";
import { CreateFileModal } from "./CreateFileModal";
import { CreateDirectoryModal } from "./CreateDirectoryModal";

const getStorageKey = (path: string) => `directory_state_${path}`;

export const Level: React.FC<{ path: string }> = ({ path }) => {
  const [directories, setDirectories] = useState<
    { name: string; open: boolean }[]
  >([]);
  const [files, setFiles] = useState<string[]>([]);
  const [creatingFile, setCreatingFile] = useState<string | null>(null);
  const [creatingDirectory, setCreatingDirectory] = useState<string | null>(
    null
  );

  const [lastUpdated, setLastUpdated] = useState(Date.now());

  const updateFolder = useCallback(() => {
    setLastUpdated(Date.now());
    const pathStr = path.replace("./files", "");
    MauriceApi.postApiListFilesAndDirectories({ path: pathStr }).then(
      (response) => {
        const storedState = JSON.parse(
          localStorage.getItem(getStorageKey(path)) || "{}"
        );
        setDirectories(
          response.files_and_directories.directories.map((directory) => ({
            name: directory,
            open: storedState[directory] || false,
          }))
        );
        setFiles(response.files_and_directories.files);
      }
    );
  }, [path]);

  useEffect(() => {
    updateFolder();
  }, [updateFolder]);

  const openDirectory = useCallback(
    (i: number) => {
      const newDirectories = [...directories];
      newDirectories[i].open = !newDirectories[i].open;
      setDirectories(newDirectories);

      // Save the new state to localStorage
      const stateToSave = newDirectories.reduce((acc, dir) => {
        acc[dir.name] = dir.open;
        return acc;
      }, {} as Record<string, boolean>);
      localStorage.setItem(getStorageKey(path), JSON.stringify(stateToSave));
    },
    [directories, path]
  );

  const handleCreateFile = useCallback(
    (newFilePath: string) => {
      updateFolder();
      // If the new file is created in a subdirectory, open that directory
      const parentDir = newFilePath.substring(0, newFilePath.lastIndexOf("/"));
      const dirIndex = directories.findIndex((dir) => dir.name === parentDir);
      if (dirIndex !== -1) {
        openDirectory(dirIndex);
      }
    },
    [updateFolder, directories, openDirectory]
  );

  const handleCreateDirectory = useCallback(() => {
    updateFolder();
  }, [updateFolder]);

  return (
    <Box>
      <CreateFileModal
        open={creatingFile !== null}
        onClose={() => setCreatingFile(null)}
        path={creatingFile}
        onCreateFile={handleCreateFile}
      />
      <CreateDirectoryModal
        open={creatingDirectory !== null}
        onClose={() => setCreatingDirectory(null)}
        path={creatingDirectory}
        onCreateDirectory={handleCreateDirectory}
      />
      <Box ml={5}>
        {files.map((file) => (
          <FileItem key={file} path={file} updateFolder={updateFolder} />
        ))}
      </Box>
      {directories.map((directory, i) => (
        <DirectoryItem
          updateFolder={updateFolder}
          key={directory.name + lastUpdated}
          directory={directory}
          onOpen={() => openDirectory(i)}
          onCreateFile={() => setCreatingFile(directory.name)}
          onCreateDirectory={() => setCreatingDirectory(directory.name)}
        >
          {directory.open && (
            <Box ml={4}>
              <Level path={directory.name} />
            </Box>
          )}
        </DirectoryItem>
      ))}
    </Box>
  );
};
