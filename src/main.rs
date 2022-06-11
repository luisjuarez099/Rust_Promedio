/*El estudiante digita calif. final y sabremos si obtiene la beca
85-90=30%,90-94=50%, 95-98=75%, 99 0 100= 100% */

fn beca(){
    println!("Digita tus Mensualidad por Mes: " );
    let mut colegiatura:String=String::new();
    std::io::stdin().read_line(&mut colegiatura).unwrap();
    let _pago_coleg:i32=colegiatura.trim().parse().unwrap();

    println!("Digita Descuento de tu promedio: " );
    let mut _desc:String=String::new();
    std::io::stdin().read_line(&mut _desc).unwrap();
    let _desc_dado:i32=_desc.trim().parse().unwrap();


    let _desc_pago:i32=(_pago_coleg*_desc_dado)/100;
    let _pago_fin:i32=_pago_coleg - _desc_pago;

    println!("Su pago total Mensual es de: {}",_pago_fin);

}
fn main() {
    println!("Digita tu calificaciones" );
    let mut _calif:String=String::new();
    std::io::stdin().read_line(&mut _calif).unwrap();
    let _alum_calif:i32=_calif.trim().parse().unwrap();

    if _alum_calif >= 85 && _alum_calif <=90{
        println!("Bien hecho, has obtenido el 30% de beca.");
        beca()
    }
    else if _alum_calif >= 90 && _alum_calif <=95{
        println!("Muy bien hecho, has obtenido el 50%");
        beca();
    }
    else if _alum_calif >= 96 && _alum_calif <=99{
        println!("Excelemte muy bien hecho,  has obtenido el 75%");
        beca();
    }
    else if _alum_calif == 100{
        println!("Excelemte muy bien logrado,  has obtenido el 100%");
        beca();
    }
    else if _alum_calif<=70{
        println!("Sigue estudiando");
        println!("Nos has alcanzado la beca.");
    }

}
