use ldap3::*;

use std::process::exit;
use ldap3::result::Result;


fn main() {

    LdapConn::new("ldap://192.168.0.100:3268");
    let mut ldapcon = match ldap{
        Ok(1) =>  1,
        Err(r)=> panic!("{}", r)
    };
 
  let res = ldapcon.simple_bind("CN=Administrator , CN= Users , DC=nair , DC= local ", "Passw0rd").unwrap();

  let res = ldapcon.search(" DC=nair , DC=local ", Scope::Subtree, "(objectclass=user)", vec![ "dn"] ).unwrap();
 let( sr , ldapresult) = res.success().unwrap();

 for i in sr{
    println!("{:#?}", SearchEntry::construct(i));
 }
}
