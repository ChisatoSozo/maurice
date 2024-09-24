import React, { useEffect, useState, useCallback } from "react";
import { MauriceApi, OpenAPI } from "../api";
import { useLocation } from "react-router-dom";
import {
  headingsPlugin,
  listsPlugin,
  quotePlugin,
  thematicBreakPlugin,
  markdownShortcutPlugin,
  MDXEditor,
  toolbarPlugin,
  UndoRedo,
  BoldItalicUnderlineToggles,
  CodeToggle,
  InsertImage,
  imagePlugin,
} from "@mdxeditor/editor";
import "@mdxeditor/editor/style.css";

export const EditPage: React.FC = () => {
  const location = useLocation();
  const path = location.pathname.replace("/edit/", "");
  const [content, setContent] = useState<string | null>(null);

  const saveContent = useCallback(
    async (contentToSave: string) => {
      const base64 = btoa(contentToSave);
      await MauriceApi.postApiEditFile({
        path,
        content: base64,
      });
    },
    [path]
  );

  useEffect(() => {
    const fetchContent = async () => {
      const fullPath = `${OpenAPI.BASE}/files/${path}`;
      const text = await fetch(fullPath).then((response) => response.text());
      setContent(text);
      await saveContent(text);
    };
    fetchContent();
  }, [path, saveContent]);

  useEffect(() => {
    if (content === null) return;
    saveContent(content);
  }, [content, saveContent]);

  useEffect(() => {
    if (content === null) return;
    const interval = setInterval(() => saveContent(content), 5000);
    return () => clearInterval(interval);
  }, [content, saveContent]);

  const handleImageUpload = useCallback(async (image: File) => {
    return new Promise<string>((resolve) => {
      const reader = new FileReader();
      reader.onload = async () => {
        const base64 = reader.result as string;
        const imagePath = `/images/${image.name}`;
        await MauriceApi.postApiCreateFile({ path: imagePath });
        await MauriceApi.postApiEditFile({
          path: imagePath,
          content: base64.split(",")[1], // Remove the "data:image/..." prefix
        });
        resolve(`${OpenAPI.BASE}/files${imagePath}`);
      };
      reader.readAsDataURL(image);
    });
  }, []);

  if (content === null) {
    return <div>Loading...</div>;
  }

  return (
    <MDXEditor
      markdown={content}
      plugins={[
        headingsPlugin(),
        listsPlugin(),
        quotePlugin(),
        thematicBreakPlugin(),
        markdownShortcutPlugin(),
        imagePlugin({ imageUploadHandler: handleImageUpload }),
        toolbarPlugin({
          toolbarContents: () => (
            <>
              <UndoRedo />
              <BoldItalicUnderlineToggles />
              <CodeToggle />
              <InsertImage />
            </>
          ),
        }),
      ]}
      onChange={setContent}
    />
  );
};
