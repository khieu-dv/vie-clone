use crate::{
    config::download_config::IPCDownloadConfig,
    logger::logger::IPCLogger,
    media::options::ProcessOptions,
    processor::processor::Processor,
    utils::{
        directory::{create_directory, remove_directory},
        file::{get_files, get_mediasafe_filename},
    },
};

use super::converter::Converter;

pub struct GifConverter {
    cfg: IPCDownloadConfig,
    opts: ProcessOptions,
}

impl Converter for GifConverter {
    fn new(config: &IPCDownloadConfig, options: &ProcessOptions) -> Self {
        Self {
            cfg: config.clone(),
            opts: options.clone(),
        }
    }

    fn convert(&self, logger: &IPCLogger) {
        self.init_dir(&self.opts.path_work);

        for input_file in get_files(&format!("{}/download", self.opts.path_work)) {
            let _ = remove_directory(&format!("{}/convert_temp", self.opts.path_work));
            let _ = create_directory(&format!("{}/convert_temp", self.opts.path_work));

            let _ = Processor::new(logger, &self.opts.bin_ffmpeg, &{
                let mut args: Vec<String> = Vec::new();

                args.push("-y".to_string());

                args.push("-i".to_string());
                args.push(input_file.clone());

                args.push("-c:v".to_string());
                args.push("libx264".to_string());

                args.push("-an".to_string());

                if self.cfg.settings.vbr_set_bitrate_enable
                    && !self.cfg.settings.vbr_set_bitrate.is_empty()
                {
                    args.push("-b:v".to_string());
                    args.push(self.cfg.settings.vbr_set_bitrate.clone());
                }

                if self.cfg.settings.size_change_enable
                    && self.cfg.settings.size_change_width.parse::<i32>().is_ok()
                    && self.cfg.settings.size_change_height.parse::<i32>().is_ok()
                {
                    args.push("-vf".to_string());
                    args.push(format!(
                        "scale={}:{},setsar=1",
                        self.cfg.settings.size_change_width, self.cfg.settings.size_change_height
                    ));
                }

                if self.cfg.settings.fps_change_enable
                    && self
                        .cfg
                        .settings
                        .fps_change_framerate
                        .parse::<i32>()
                        .is_ok()
                {
                    args.push("-r".to_string());
                    args.push(self.cfg.settings.fps_change_framerate.clone());
                }

                if self.cfg.settings.trim_enable {
                    if !self.cfg.settings.trim_from_start_enable {
                        args.push("-ss".to_string());
                        args.push(self.cfg.settings.trim_start.clone());
                    }
                    if !self.cfg.settings.trim_to_end_enable {
                        args.push("-to".to_string());
                        args.push(self.cfg.settings.trim_end.clone());
                    }
                }

                args.push(format!("{}/convert_temp/video.mp4", self.opts.path_work));

                args
            })
            .start();

            let _ = Processor::new(logger, &self.opts.bin_ffmpeg, &{
                let mut args: Vec<String> = Vec::new();

                args.push("-y".to_string());

                args.push("-i".to_string());
                args.push(format!("{}/convert_temp/video.mp4", self.opts.path_work));

                args.push("-vf".to_string());
                args.push("palettegen".to_string());

                args.push(format!("{}/convert_temp/palette.png", self.opts.path_work));

                args
            })
            .start();

            let _ = Processor::new(logger, &self.opts.bin_ffmpeg, &{
                let mut args: Vec<String> = Vec::new();

                args.push("-y".to_string());

                args.push("-i".to_string());
                args.push(format!("{}/convert_temp/video.mp4", self.opts.path_work));

                args.push("-i".to_string());
                args.push(format!("{}/convert_temp/palette.png", self.opts.path_work));

                args.push("-lavfi".to_string());
                args.push("paletteuse".to_string());

                args.push(format!(
                    "{}/convert/{}.gif",
                    &self.opts.path_work,
                    get_mediasafe_filename(&input_file, false)
                ));

                args
            })
            .start();
        }
    }
}
