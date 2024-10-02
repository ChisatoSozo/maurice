import {
  headingsPlugin,
  listsPlugin,
  quotePlugin,
  thematicBreakPlugin,
  markdownShortcutPlugin,
  MDXEditor,
  toolbarPlugin,
  BoldItalicUnderlineToggles,
  InsertImage,
  MDXEditorMethods,
  // imagePlugin,
  // MDXEditorMethods,
} from "@mdxeditor/editor";
import { Box, Container } from "@mui/material";
import "@mdxeditor/editor/style.css";
import { useCallback, useEffect, useRef, useState } from "react";
import { useLocation } from "react-router-dom";
import { MauriceApi, OpenAPI } from "../api";
import { useCompletion } from "../hooks/useCompletion";
// import { LoadCompletionModel } from "../components/LoadCompletionModel";

const useInitialMarkdown = (path: string) => {
  const [markdown, setMarkdown] = useState<string | null>(null);
  useEffect(() => {
    const fetchMarkdown = async () => {
      const res = await fetch(
        OpenAPI.BASE + "/files" + path + "?" + Date.now()
      );
      const text = await res.text();
      const decoded = decodeURIComponent(text);
      setMarkdown(decoded);
    };
    fetchMarkdown();
  }, [path]);
  return markdown;
};

const COMPLETION_WRAPPED_START = "*";
const COMPLETION_WRAPPED_END = "*";

export const EditPage: React.FC = () => {
  const path = useLocation().pathname.replace("/edit", "");
  const initialMarkdown = useInitialMarkdown(path);
  const { component, updateContent, completion, acceptOrRejectCompletion } =
    useCompletion();
  const [preCompletionMarkdown, setPreCompletionMarkdown] = useState<
    string | null
  >(null);
  const editorRef = useRef<MDXEditorMethods | null>(null);

  useEffect(() => {
    if (!completion) {
      return;
    }
    const preCompletionMarkdown = editorRef.current?.getMarkdown() || "";
    setPreCompletionMarkdown(preCompletionMarkdown);
    //if the completion starts with whitespace, move the whitespace before the COMPLETION_WRAPPED_START
    const whitespaceAtTheStart = (completion.match(/^\s+/)?.[0] || "").replace(
      " ",
      "&#x20;"
    );
    const completionWithoutWhitespace = completion.replace(/^\s+/, "");

    editorRef.current?.insertMarkdown(
      whitespaceAtTheStart +
        COMPLETION_WRAPPED_START +
        completionWithoutWhitespace +
        COMPLETION_WRAPPED_END
    );
  }, [completion]);

  const newMarkdown = useCallback(
    async (markdown: string) => {
      console.log(markdown);
      if (completion) {
        return;
      }
      updateContent(markdown);
      await MauriceApi.postApiEditFile({
        path: decodeURIComponent(path),
        content: btoa(encodeURIComponent(markdown)),
      });
    },
    [completion, path, updateContent]
  );

  const rejectCompletion = useCallback(() => {
    if (!preCompletionMarkdown || !completion) {
      return;
    }
    acceptOrRejectCompletion();
    editorRef.current?.focus();
    editorRef.current?.setMarkdown("");
    editorRef.current?.insertMarkdown(preCompletionMarkdown || "");
    setPreCompletionMarkdown(null);
  }, [acceptOrRejectCompletion, completion, preCompletionMarkdown]);

  const keyDown = useCallback(
    (e: React.KeyboardEvent<HTMLDivElement>) => {
      if (completion) {
        //if tab is pressed, accept completion
        if (e.key === "Tab" || e.key === "@") {
          e.preventDefault();
          updateContent(preCompletionMarkdown + completion);
          acceptOrRejectCompletion();
          editorRef.current?.focus();
          editorRef.current?.setMarkdown("");
          editorRef.current?.insertMarkdown(preCompletionMarkdown + completion);
        } else {
          rejectCompletion();
        }
        setPreCompletionMarkdown(null);
      }
    },
    [
      acceptOrRejectCompletion,
      completion,
      preCompletionMarkdown,
      rejectCompletion,
      updateContent,
    ]
  );

  if (initialMarkdown === null) {
    return <Container>Loading...</Container>;
  }

  return (
    <Container
      onKeyDown={(e) => keyDown(e)}
      onMouseEnter={() => rejectCompletion()}
      onMouseMove={() => rejectCompletion()}
      onTouchStart={() => rejectCompletion()}
      onTouchMove={() => rejectCompletion()}
    >
      <MDXEditor
        contentEditableClassName="mdxeditor"
        ref={editorRef}
        markdown={initialMarkdown}
        onChange={newMarkdown}
        plugins={[
          headingsPlugin(),
          listsPlugin(),
          quotePlugin(),
          thematicBreakPlugin(),
          markdownShortcutPlugin(),
          // imagePlugin({ imageUploadHandler: handleImageUpload }),
          toolbarPlugin({
            toolbarContents: () => (
              <>
                <Box
                  display="flex"
                  flexDirection="row"
                  alignItems="center"
                  width="100%"
                  overflow="hidden"
                >
                  <Box display="flex">
                    <BoldItalicUnderlineToggles />
                    <InsertImage />
                  </Box>
                  <Box flexGrow={1} />
                  {component}
                </Box>
              </>
            ),
          }),
        ]}
      />
    </Container>
  );
};
