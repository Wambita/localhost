use {
    super::{
        external::{
            Http,
            Upload,
        },
        Route,
        Server,
    },
    crate::{
        message::{
            Method,
            Request,
            Resource,
            Response,
        },
        utils::HttpStatus,
    },
    std::io::Write,
};

impl Server {
    pub fn get_error_page(&self, error_code: u16) -> Option<&String> {
        match &self.error_pages {
            Some(pages) => pages.get(&error_code.to_string()),
            None => None,
        }
    }

    pub fn get_route(&self, path: &str) -> Option<&Route> {
        if let Some(routes) = &self.routes {
            for route in routes {
                if let Some(route_path) = &route.path {
                    if route_path == path {
                        return Some(route);
                    }
                }
            }
        }
        None
    }

    pub(crate) fn direct(
        &self,
        request: &Request,
        stream: &mut impl Write,
        addr: &str,
    ) -> Response {
        match &request.resource {
            Resource::Path(s) => {
                if let Some(route_p) = self.get_route(s) {
                    if let Some(methods) = &route_p.methods {
                        if !methods.contains(&request.method.to_string()) {
                            dbg!("Method not allowed", s, &request.method);
                            return match self.get_error_page(405) {
                                Some(page) => Response::err(page, None),
                                None => Response::from(HttpStatus::from(405)),
                            };
                        }
                    }
                }

                if s == "/auth" && request.method == Method::POST {
                    dbg!("Redirecting to login page", s);
                    return Http::set_cookie().unwrap_or_else(|e| {
                        match self.get_error_page(e.status_code) {
                            Some(page) => Response::err(page, None),
                            None => Response::from(e),
                        }
                    });
                }

                if !self.check_session(s, &request) {
                    if let Some((route, fichier)) = self.redirect(s) {
                        return Http::serve_auth(&route, &fichier, &self, stream, addr)
                            .unwrap_or_else(
                                |e| match self.get_error_page(e.status_code) {
                                    Some(page) => Response::err(page, None),
                                    None => Response::from(e),
                                },
                            );
                    }
                    else {
                        return match self.get_error_page(404) {
                            Some(page) => Response::err(page, None),
                            None => Response::from(HttpStatus::from(404)),
                        };
                    }
                };

                match request.method {
                    Method::GET => Http::handle(&request, self),
                    Method::POST => Upload::handle(&request),
                    Method::DELETE => self.delete(&request),
                    _ => Err(HttpStatus::from(405)),
                }
            }
        }
        .unwrap_or_else(
            |e| match self.get_error_page(e.status_code) {
                Some(page) => Response::err(page, None),
                None => Response::from(e),
            },
        )
    }

    pub fn find_default_file(&self, path: &str) -> Option<String> {
        if let Some(routes) = &self.routes {
            for route in routes {
                if let Some(route_path) = &route.path {
                    if route_path == path {
                        return route.default_file.clone();
                    }
                }
            }
        }
        None
    }

    pub fn is_configured_path(&self, path: &str) -> bool {
        if let Some(routes) = &self.routes {
            for route in routes {
                if let Some(route_path) = &route.path {
                    if route_path == path {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn redirect(&self, path: &str) -> Option<(String, String)> {
        if let Some(routes) = &self.routes {
            for route in routes {
                if let Some(route_path) = &route.path {
                    if route_path == path {
                        if let Some(redirect) = &route.redirect {
                            if let Some((key, value)) = redirect.iter().next() {
                                return Some((key.clone(), value.clone()));
                            }
                        }
                    }
                }
            }
        }
        None
    }

    pub fn reformat_path(&self, path: &str) -> String {
        if path.starts_with('/') {
            path.to_string()
        }
        else {
            format!("/{}", path)
        }
    }
}
