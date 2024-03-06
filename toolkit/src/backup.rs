//backup.rs
pub fn backup_data(name_var:&str) -> &'static str{
   
let ping_test_list_ip = r#"{"Proveedor_FIBEX"=["8.8.8.8","45.182.140.24","10.0.0.8:22"],
"Proveedor_Digitel_Carabobo"=["8.8.8.8","38.183.113.0","10.0.0.6:22"],
"Proveedor_Digitel_Aragua"=["1.1.1.1","38.183.114.0","10.0.0.5:22"],
"Proveedor_InterCarabobo"=["8.8.4.4","206.1.88.32","10.0.0.8:22"],
"Proveedor_Inter_Aragua"=["1.1.1.1","200.82.153.240","10.0.0.5:22"],
"TD_Pto_Digitel"=["172.16.8.2","172.16.8.1","10.0.0.6:22"],
"TD_Pto_Fibex"=["172.16.0.17","172.16.0.19","10.1.96.1:22"],
"TD_Fibex_Guacara"=["172.16.2.78","172.16.2.77","10.0.1.20:2022"],
"TD_Fibex_Mirador"=["172.16.0.61","172.16.0.62","10.1.44.1:22"],
"TD_Fibex_VistaHermosa"=["172.16.0.49","172.16.0.51","10.1.4.1:22"],
"TD_Fibex_CaribeMorita"=["172.16.0.49","172.16.0.50","10.0.0.5:22"],
"TD_Int_Caribe_VistaHer"=["172.16.4.90","172.16.4.89","10.0.0.5:22"],
"TD_Int_Dayco_Mirador"=["172.16.0.129","172.16.0.130","10.1.44.1:22"],
"TD_Int_Dayco_Guacara"=["172.16.0.125","172.16.0.126","10.10.44.1:22"],
"TD_Int_Dayco_Parques"=["172.16.0.121","172.16.0.122","10.10.48.1:22"],
"TD_Int_Dayco_Fundacion"=["172.16.2.45","172.16.2.46","10.1.51.1:22"],
"TD_Int_Dayco_Paseo"=["172.16.0.73","172.16.0.74","45.182.141.53:22"],
"TD_Int_VLAN3000_Parral_Dayco"=["172.16.2.1","172.16.2.2","10.1.36.1:22"],
"TD_Int_VLAN3006_Copey_Dayco"=["172.16.0.25","172.16.0.26","10.1.32.1:22"]}"#;

let report_template = 
r#"#01 ▶️ _Status de la red y de los reportes de fallas del servicio_ * _date_ _hour_am_pm_ *
#02
#03 📌 *Chats en espera:* 
#04 	*Bandeja de especialistas:* 
#05 	*Bandeja de ATC:* 
#06
#07 📌 *Tickets en progreso por Especialistas:* 
#08
#09 * Latencia en los proveedores y Servicios de transporte de datos:*
#10
#11 _Proveedores:_
$12	🌐 *Fibex:* Proveedor_FIBEX
$13	🌐 *Digitel:* Proveedor_Digitel_Carabobo
$14	🌐 *Digitel Aragua:* Proveedor_Digitel_Aragua
$15	🌐 *Inter Carabobo:* Proveedor_InterCarabobo
$16	🌐 *Inter Aragua:* Proveedor_Inter_Aragua
#17
#18 _Transportes:_  Externo: 🟤 Interno:🔵
$19	🟤 *Puerto Cabello - Digitel:* TD_Pto_Digitel
$20	🟤 *Puerto Cabello - Fibex:* TD_Pto_Fibex
$21	🟤 *Guacara - Fibex:* TD_Fibex_Guacara
$22	🟤 *Mirador - Fibex:* TD_Fibex_Mirador
$23	🟤 *Netcom Plus Maracay - VistaHermosa:* TD_Fibex_VistaHermosa
$24	🟤 *Netcom Plus Maracay - Caribe/Morita:* TD_Fibex_CaribeMorita
$25	🔵 *Netcom Plus Maracay - Caribe to Vista Hermosa:* TD_Int_Caribe_VistaHer
$26	🔵 *Dayco - Mirador:* TD_Int_Dayco_Mirador
$27	🔵 *Dayco - Guacara:* TD_Int_Dayco_Guacara
$28	🔵 *Dayco - Los Parques:* TD_Int_Dayco_Parques
$29	🔵 *Dayco - Fundacion Mendoza:* TD_Int_Dayco_Fundacion
$30	🔵 *Dayco - Paseo las industrias:* TD_Int_Dayco_Paseo
$31	🔵 *Siklu Copey - Parral:* TD_Int_VLAN3000_Parral_Dayco"#;

let ftth_nodes = 
r#"{"CEC":"10.1.20.1:22",
"IL":"10.1.8.1:22",
"MS":"10.10.56.1:22",
"TE":"10.0.3.2:22",
"MRD":"10.1.44.1:22"}"#;

let rf_nodes =
r#"{"Castillito" : ["1","52","10.1.52.1:22"],
"Castellana" : ["1","60","10.1.60.1:22"],
"Copei" : ["1","32","10.1.32.1:22"],
"Copei_Arriba" : ["10","52","10.1.32.1:22"],
"Colina" : ["1","40","10.1.40.1:22"],
"La_esmeralda" : ["1","56","10.1.56.1:22"],
"Flor_amarillo" : ["10","40","10.10.40.1:22"],
"Guacara" : ["10","44","10.10.44.1:22"],
"Islalarga" : ["1","8","10.1.8.1:22"],
"Mirador" : ["1","44","10.1.44.1:22"],
"Paseo" : ["10","36","10.10.36.1:22"],
"Parques" : ["10","48","10.10.48.1:22"],
"Parral" : ["1","36","10.1.36.1:22"],
"San_Andres" : ["10","32","10.10.32.1:22"],
"Torre_Ejecutiva" : ["1","96","10.1.96.1:22"],
"Xian" : ["1","48","10.1.48.1:22"]}"#;

let default_config =
r#"{"default_ro_test_login":"10.0.0.8:22",
    "default_ro_search_segments":"10.0.0.8:22"}"#;

match name_var {
    "ping_test_list_ip"=> {return ping_test_list_ip;} ,
    "report_template"=> {return report_template;},
    "ftth_nodes"=> {return ftth_nodes;},
    "rf_nodes"=>{return rf_nodes;},
    "default_config" => {return default_config;}
    _=>{return "error";}

}

}


