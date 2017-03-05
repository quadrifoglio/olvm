/*
 * Backend image management
 */

use common::{Context, Result, Error};
use common::structs::Image;

pub fn script_create(ctx: &Context, img: &Image) -> Result<()> {
    let backend = try!(ctx.conf.get_backend(img.backend.as_str()).ok_or(Error::new("Invalid or unknown backend")));

    if let Some(_) = backend.image.create {
        // TODO: Execute script
    }

    Ok(())
}

pub fn script_delete(ctx: &Context, img: &Image) -> Result<()> {
    let backend = try!(ctx.conf.get_backend(img.backend.as_str()).ok_or(Error::new("Invalid or unknown backend")));

    if let Some(_) = backend.image.delete {
        // TODO: Execute script
    }

    Ok(())
}
