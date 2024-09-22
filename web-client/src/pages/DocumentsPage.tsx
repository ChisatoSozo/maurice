import { useEffect, useRef, useState } from "react";
import { MauriceApi } from "../api";

export const DocumentsPage = () => {
  const [document, setDocument] = useState<string | null>(null);
  const [localEdit, setLocalEdit] = useState<string | null>(null);
  const [serverVersion, setServerVersion] = useState<string | null>(null); // Track the server version
  const syncing = useRef(false); // Tracks if syncing is happening
  const isFetching = useRef(false); // Tracks if fetching is in progress

  // Function to sync the document
  const syncDocument = async (documentContent: string) => {
    syncing.current = true;
    const base64 = btoa(documentContent);

    try {
      await MauriceApi.postApiEditFile({
        content: base64,
        path: "tmp.txt",
      });
    } catch (error) {
      console.error("Error syncing document:", error);
    } finally {
      syncing.current = false;
    }
  };

  // Handle document changes (local edits)
  const handleDocumentChange = (newText: string) => {
    setLocalEdit(newText); // Track the user's local edit
    setDocument(newText); // Update the displayed document
  };

  // Debounced effect for syncing document changes
  useEffect(() => {
    if (localEdit !== null) {
      const timeout = setTimeout(() => {
        syncDocument(localEdit); // Sync the document after a short delay
        setLocalEdit(null); // Clear the localEdit once synced
      }, 300); // 300ms debounce for batching edits

      return () => clearTimeout(timeout); // Clear timeout on component unmount
    }
  }, [localEdit]);

  // Frequent fetching logic
  useEffect(() => {
    const interval = setInterval(async () => {
      if (syncing.current || isFetching.current) return; // Don't fetch while syncing or fetching

      isFetching.current = true;
      try {
        const response = await fetch("http://192.168.2.56:8080/files/tmp.txt");
        const text = await response.text();

        // Check if server text is different from what we have
        if (text !== serverVersion) {
          setServerVersion(text); // Update the server version

          // Only update the document if there's no unsynced local edit
          if (!localEdit) {
            setDocument(text);
          }
        }
      } catch (error) {
        console.error("Error fetching document:", error);
      } finally {
        isFetching.current = false;
      }
    }, 10); // Increased fetch interval to 1000ms to reduce load

    return () => clearInterval(interval);
  }, [localEdit, serverVersion]);

  return (
    <textarea
      style={{
        width: "calc(100% - 7px)",
        height: "calc(100% - 10px)",
        fontSize: "1.5em",
      }}
      value={document || ""}
      onChange={(e) => {
        handleDocumentChange(e.target.value);
      }}
    />
  );
};
