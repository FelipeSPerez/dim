import { useEffect, useState, useContext } from "react";
import { useSelector } from "react-redux";

import { VideoPlayerContext } from "./Context";

import JASSUB from "jassub";

import "./Subtitles.scss";

function VideoSubtitles() {
  const { video, subtitle, token } = useSelector((store) => ({
    video: store.video,
    subtitle: store.video.tracks.subtitle,
    token: store.auth.token,
  }));

  const currentSub = subtitle.list[subtitle.current];

  const isAssEnabled = localStorage.getItem("enable_ssa") === "true";
  const isAss = !!(isAssEnabled && currentSub?.chunk_path?.endsWith("ass"));
  const [jassub, setJASSUB] = useState();
  const { videoRef } = useContext(VideoPlayerContext);

  useEffect(() => {
    if (
      jassub ||
      !video.textTrackEnabled ||
      video.prevSubs === subtitle.current ||
      !isAss ||
      !videoRef
    )
      return;

    console.log("[INFO] Loading ASS subtitle");

    const chunk_path = `//${window.location.host}/api/v1/stream/${
      subtitle.list[subtitle.current].chunk_path
    }`;

    JASSUB._test();

    const options = {
      video: videoRef.current,
      dropAllBlur: !JASSUB._supportsSIMD,
      workerUrl: new URL(
        "jassub/dist/jassub-worker.js",
        import.meta.url
      ).toString(),
      wasmUrl: new URL(
        "jassub/dist/jassub-worker.wasm",
        import.meta.url
      ).toString(),
      modernWasmUrl: new URL(
        "jassub/dist/jassub-worker-modern.wasm",
        import.meta.url
      ).toString(),
      availableFonts: { "liberation sans": "/static/default.woff2" },
      fonts: ["/static/default.woff2"],
    };

    const instance = new JASSUB(options);
    fetch(chunk_path, { headers: { Authorization: token } })
      .then((res) => res.text())
      .then((content) => instance.setTrack(content));
    setJASSUB(instance);

    return () => {
      console.log("[subtitle] disposing of jassub ctx");
      if (jassub) jassub.destroy();
    };
  }, [video, videoRef, subtitle, isAss, setJASSUB, jassub, token]);

  useEffect(() => {
    if (
      !jassub ||
      !video.textTrackEnabled ||
      video.prevSubs === subtitle.current ||
      !isAss
    )
      return;

    const chunk_path = `//${window.location.host}/api/v1/stream/${
      subtitle.list[subtitle.current].chunk_path
    }`;
    fetch(chunk_path, { headers: { Authorization: token } })
      .then((res) => res.text())
      .then((content) => jassub.setTrack(content));
  }, [jassub, video.textTrackEnabled, video.prevSubs, subtitle, isAss, token]);

  useEffect(() => {
    if (jassub && !isAss) {
      console.log("[subtitle] disposing of jassub ctx");
      jassub.destroy();
      setJASSUB(null);
    }
  }, [jassub, setJASSUB, isAss]);

  return null;
}

export default VideoSubtitles;
