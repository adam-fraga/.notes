/*
Credentials here is nested into models modules so we have to specify
the rel or abs path to Credentials
*/
pub fn login(creds: models::Credentials) {
    //Relative path
    //authenticate...

    //get_user is private here so we have to change it to public
    crate::database::get_user(); //Absolute path
}

fn logout() { /*logout the user*/
}

/*
Here we have the content of auth_utils and the submodule of auth_utils are
store in the folder auth_utils, Normaly rust use à file called mod.rs when
we have nested module to split into multiple file but this approche is complicate
when you have to edit multiple  mod.rs file (same name for all modules)
*/
pub mod models;
