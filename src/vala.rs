/* lib.rs
 *
 * Copyright 2024 Mario Daniel Ruiz Saavedra <desiderantes93@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3.0 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use std::fs;
use zed::serde_json;
use zed::LanguageServerId;
use zed_extension_api::{
    self as zed, lsp::Completion, settings::LspSettings, CodeLabel, CodeLabelSpan, Result,
};

enum VlsKind {
    ValaLanguageServer,
    GVLS,
}

impl Default for VlsKind {
    fn default() -> Self {
        VlsKind::ValaLanguageServer
    }
}

impl Clone for VlsKind {
    fn clone(&self) -> Self {
        match self {
            VlsKind::ValaLanguageServer => VlsKind::ValaLanguageServer,
            VlsKind::GVLS => VlsKind::GVLS,
        }
    }
}

impl VlsKind {
    fn exec_name(&self) -> &'static str {
        match self {
            VlsKind::ValaLanguageServer => "vala-language-server",
            VlsKind::GVLS => "gvls",
        }
    }
}

struct ValaExtension {
    cached_binary_path: Option<(VlsKind, String)>,
}

impl ValaExtension {
    fn language_server_binary_path(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<(VlsKind, String)> {
        //already downloaded?
        if let Some((kind, path)) = &self.cached_binary_path {
            if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                return Ok((kind.clone(), path.clone()));
            }
        }
        //vala-language-server binary in PATH?
        if let Some(path) = worktree.which(VlsKind::ValaLanguageServer.exec_name()) {
            self.cached_binary_path = Some((VlsKind::ValaLanguageServer, path.clone()));
            return Ok((VlsKind::ValaLanguageServer, path));
        }
        //gvls binary in PATH?
        if let Some(path) = worktree.which(VlsKind::GVLS.exec_name()) {
            self.cached_binary_path = Some((VlsKind::GVLS, path.clone()));
            return Ok((VlsKind::GVLS, path));
        }

        //let (platform, arch) = zed::current_platform();

        let error_message = format!(
            "Could not find vala-language-server or gvls binary in PATH. \
            Please install vala-language-server or gvls and try again."
        );
        zed::set_language_server_installation_status(
            &language_server_id,
            &zed::LanguageServerInstallationStatus::Failed(error_message.clone()),
        );
        return Err(error_message);
    }
}

impl zed::Extension for ValaExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }
    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let (kind, path) = self.language_server_binary_path(language_server_id, worktree)?;
        return match kind {
            VlsKind::ValaLanguageServer => Ok(zed::Command {
                command: path,
                args: vec![],
                env: vec![("G_MESSAGES_DEBUG".to_string(), "all".to_string())],
            }),
            VlsKind::GVLS => Ok(zed::Command {
                command: path,
                args: vec![],
                env: vec![("G_MESSAGES_DEBUG".to_string(), "all".to_string())],
            }),
        };
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let settings = LspSettings::for_worktree("vala-language-server", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone())
            .unwrap_or_default();

        Ok(Some(serde_json::json!({
            "vala": settings
        })))
    }

    fn label_for_completion(
        &self,
        _language_server_id: &LanguageServerId,
        completion: Completion,
    ) -> Option<zed::CodeLabel> {
        let kind = match completion.kind {
            Some(zed::lsp::CompletionKind::Function) | Some(zed::lsp::CompletionKind::Method) => {
                match completion.detail {
                    Some(a) => a,
                    _ => completion.label,
                }
            }
            _ => match completion.detail {
                Some(a) => format!("{} {}", completion.label, a),
                _ => completion.label,
            },
        };

        Some(CodeLabel {
            spans: vec![CodeLabelSpan::code_range(0..kind.len())],
            filter_range: (0..kind.len()).into(),
            code: kind,
        })
    }
}

zed::register_extension!(ValaExtension);
