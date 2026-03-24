use super::ProfileContext;
use super::ProfileType;
use super::StreamType;
use super::TranscodingProfile;

use crate::error::NightfallError;

#[derive(Debug)]
pub struct AacTranscodeProfile;

impl TranscodingProfile for AacTranscodeProfile {
    fn profile_type(&self) -> ProfileType {
        ProfileType::Transcode
    }

    fn stream_type(&self) -> StreamType {
        StreamType::Audio
    }

    fn name(&self) -> &str {
        "AacTranscodeProfile"
    }

    fn build(&self, ctx: ProfileContext) -> Option<Vec<String>> {
        let start_num = ctx.output_ctx.start_num.to_string();
        let stream = format!("0:{}", ctx.input_ctx.stream);
        let init_seg = format!("{}_init.mp4", &start_num);
        let seg_name = format!("{}/%d.m4s", ctx.output_ctx.outdir);
        let outdir = format!("{}/playlist.m3u8", ctx.output_ctx.outdir);

        // NOTE: might need flags -fflages +genpts if seeking breaks.
        let mut args = vec![
            "-y".into(),
            "-ss".into(),
            (ctx.output_ctx.start_num * ctx.output_ctx.target_gop).to_string(),
            "-i".into(),
            ctx.file,
            "-copyts".into(),
            "-map".into(),
            stream,
            "-c:0".into(),
            "aac".into(),
        ];

        // Build the audio filter chain. Multiple -af flags are not additive; filters must be
        // combined with commas into a single filtergraph.
        let mut audio_filters: Vec<&str> = Vec::new();
        let pan_filter;
        if ctx.input_ctx.audio_channels != ctx.output_ctx.audio_channels {
            pan_filter = "pan=stereo|FL=0.5*FC+0.707*FL+0.707*BL+0.5*LFE|FR=0.5*FC+0.707*FR+0.707*BR+0.5*LFE";
            audio_filters.push(pan_filter);
        }
        // For the first segment only, force audio to start at PTS=0.  Without this, the AAC
        // encoder priming delay causes segment 0 to be shorter than target_gop, leaving a gap
        // (e.g. 0–3s of audio followed by a segment-1 TFDT at 5s) that stalls the player.
        if ctx.output_ctx.start_num == 0 {
            audio_filters.push("aresample=async=1:first_pts=0");
        }
        if !audio_filters.is_empty() {
            args.push("-af".into());
            args.push(audio_filters.join(","));
        }

        let ab = ctx.output_ctx.bitrate.unwrap_or(120_000).to_string();
        args.push("-ab".into());
        args.push(ab);

        // make_zero zeroes all timestamps unconditionally — correct for the initial segment
        // (shifts the small AAC encoder-delay negative DTS to exactly 0) but wrong for seeks
        // (it would zero out the large positive source timestamps, placing audio at t=0 in
        // the MSE SourceBuffer instead of the seek position, causing an immediate stall).
        let avoid_neg = if ctx.output_ctx.start_num == 0 {
            "make_zero"
        } else {
            "make_non_negative"
        };
        args.append(&mut vec![
            "-vsync".into(),
            "-1".into(),
            "-avoid_negative_ts".into(),
            avoid_neg.into(),
        ]);

        args.append(&mut vec![
            "-f".into(),
            "hls".into(),
            "-start_number".into(),
            start_num,
        ]);

        // needed so that in progress segments are named `tmp` and then renamed after the data is
        // on disk.
        // This in theory practically prevents the web server from returning a segment that is
        // in progress.
        args.append(&mut vec![
            "-hls_flags".into(),
            "temp_file".into(),
            "-max_delay".into(),
            "5000000".into(),
        ]);

        // these args are needed if we start a new stream in the middle of a old one, such as when
        // seeking. These args will reset the base decode ts to equal the earliest presentation
        // timestamp.
        if ctx.output_ctx.start_num > 0 {
            args.append(&mut vec![
                "-hls_segment_options".into(),
                "movflags=frag_custom+empty_moov+default_base_moof+frag_discont".into(),
            ]);
        } else {
            args.append(&mut vec![
                "-hls_segment_options".into(),
                "movflags=frag_custom+empty_moov+default_base_moof".into(),
            ]);
        }

        // args needed so we can distinguish between init fragments for new streams.
        // Basically on the web seeking works by reloading the entire video because of
        // discontinuity issues that browsers seem to not ignore like mpv.
        args.append(&mut vec!["-hls_fmp4_init_filename".into(), init_seg]);

        args.append(&mut vec![
            "-hls_time".into(),
            ctx.output_ctx.target_gop.to_string(),
            "-force_key_frames".into(),
            format!("expr:gte(t,n_forced*{})", ctx.output_ctx.target_gop),
        ]);

        args.append(&mut vec!["-hls_segment_type".into(), "1".into()]);
        args.append(&mut vec![
            "-loglevel".into(),
            "error".into(),
            "-progress".into(),
            "pipe:1".into(),
        ]);
        args.append(&mut vec!["-hls_segment_filename".into(), seg_name]);
        args.append(&mut vec![outdir]);

        Some(args)
    }

    fn supports(&self, ctx: &ProfileContext) -> Result<(), NightfallError> {
        if ctx.output_ctx.codec == "aac" {
            return Ok(());
        }

        Err(NightfallError::ProfileNotSupported(
            "Profile not supported.".into(),
        ))
    }

    fn tag(&self) -> &str {
        "aac"
    }
}
