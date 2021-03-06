use reqwest::{blocking::Client, Error};

use crate::types::{TelegraphResult, Account, AccountField};
use crate::requests::{CreateAccount, EditAccountInfo, GetAccountInfo, CreatePage};


struct MethodName {
    create_account: &'static str,
    edit_account_info: &'static str,
    get_account_info: &'static str
}

impl Default for MethodName{
    fn default() -> Self {
        MethodName {
            create_account: "https://api.telegra.ph/createAccount",
            edit_account_info: "https://api.telegra.ph/editAccountInfo",
            get_account_info: "https://api.telegra.ph/getAccountInfo"
        }
    }
}

pub struct Telegraph {
    client: Client,
    method_name: MethodName
}

impl Default for Telegraph {
    fn default() -> Self {
        Telegraph {
            client: Client::new(),
            method_name: Default::default()
        }
    }
}



impl Telegraph {
    pub fn new() -> Self {
        Telegraph::default()
    }

    pub fn create_account(&self) -> CreateAccount {
        CreateAccount::new(&self.client, "https://api.telegra.ph/createAccount")
    }

    pub fn edit_account_info(&self) -> EditAccountInfo 
    {
        EditAccountInfo::new(&self.client, "https://api.telegra.ph/editAccountInfo")
    }

    pub fn get_account_info<'get_account_info>(
        &self,
        access_token: &'get_account_info str,
        fields: impl Into<Option<Vec<AccountField>>>
    ) -> Result<Account, Error>
    {
        let params = GetAccountInfo::new(
            access_token,
            fields
        );
        let b = self.client.post(self.method_name.get_account_info).form(&params);
        let req = b.send()?;
        let json: TelegraphResult<Account> = req.json()?;
        // TODO: Handle error if ok false or result None
        Ok(json.result.unwrap_or_default())
    }

    pub fn create_page(&self) -> CreatePage {
        CreatePage::new(&self.client, "https://api.telegra.ph/createPage")
    }
}
