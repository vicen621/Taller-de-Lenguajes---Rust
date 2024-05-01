pub struct Fecha {
    dia: u32,
    mes: u32,
    año: u32,
}

impl Fecha {
    pub fn new(dia: u32, mes: u32, año: u32) -> Fecha {
        Fecha {
            dia,
            mes,
            año,
        }
    }

    pub fn es_fecha_valida(&self) -> bool {
        self.dia <= self.obtener_dias_para_mes() && self.dia > 0
    }

    pub fn es_bisiesto(&self) -> bool {
        self.año % 4 == 0
    }

    /// Devuelve la cantidad de dias que tiene el mes actual
    fn obtener_dias_para_mes(&self) -> u32 {
        if self.mes > 12 || self.mes < 1 {
            return 0;
        }

        const DIAS_POR_MES: [u32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let dias = DIAS_POR_MES[(self.mes - 1) as usize];
        // bool as u32 = if true { 1 } else { 0 }
        dias + (self.mes == 2 && self.es_bisiesto()) as u32
    }

    pub fn sumar_dias(&mut self, dias: u32) {
        let mut dias_restantes = dias;
        while dias_restantes > 0 {
            let dias_en_mes = self.obtener_dias_para_mes();
            // Se suma 1 ya que tengo que contar el dia actual
            let dias_hasta_fin_de_mes = dias_en_mes - self.dia + 1;

            if dias_hasta_fin_de_mes > dias_restantes {
                self.dia += dias_restantes;
                dias_restantes = 0;
            } else {
                dias_restantes -= dias_hasta_fin_de_mes;
                self.mes += 1;
                if self.mes > 12 {
                    self.mes = 1;
                    self.año += 1;
                }
                self.dia = 1;
            }
        }
    }

    pub fn restar_dias(&mut self, dias: u32) {
        let mut dias_restantes = dias;
        while dias_restantes > 0 {
            if self.dia > dias_restantes {
                self.dia -= dias_restantes;
                dias_restantes = 0;
            } else {
                dias_restantes -= self.dia;
                self.mes -= 1;
                if self.mes == 0 {
                    self.mes = 12;
                    self.año -= 1;
                }
                self.dia = self.obtener_dias_para_mes();
            }
        }
    }

    pub fn es_mayor(&self, una_fecha: &Fecha) -> bool {
        (self.año > una_fecha.año) || 
            (self.año == una_fecha.año && self.mes > una_fecha.mes) || 
            (self.año == una_fecha.año && self.mes == una_fecha.mes && self.dia > una_fecha.dia)
    }
}

#[test]
fn tests_es_fecha_valida() {
    assert!(Fecha::new(29, 2, 2024).es_fecha_valida());
    assert!(Fecha::new(31, 1, 2022).es_fecha_valida());
    assert!(!Fecha::new(29, 2, 2023).es_fecha_valida());
    assert!(!Fecha::new(32, 1, 2022).es_fecha_valida());
    assert!(!Fecha::new(31, 4, 2024).es_fecha_valida());
    assert!(!Fecha::new(31, 13, 2024).es_fecha_valida());
}

#[test]
fn tests_es_biciesto() {
    assert!(Fecha::new(1, 1, 2024).es_bisiesto());
    assert!(Fecha::new(1, 1, 0).es_bisiesto());
    assert!(Fecha::new(1, 1, 100).es_bisiesto());
    assert!(!Fecha::new(1, 1, 2023).es_bisiesto());
    assert!(!Fecha::new(1, 1, 2021).es_bisiesto());
}

#[test]
fn test_es_mayor() {
    let fecha1 = Fecha::new(2, 11, 2009);
    let fecha2 = Fecha::new(7, 5, 2005);

    assert!(fecha1.es_mayor(&fecha2));
    assert!(!fecha2.es_mayor(&fecha1));
}

#[test]
fn test_sumar_dias() {
    let mut fecha = Fecha::new(1, 1, 2024);
    fecha.sumar_dias(365);

    assert_eq!(fecha.dia, 31);
    assert_eq!(fecha.mes, 12);
    assert_eq!(fecha.año, 2024);

    fecha.sumar_dias(1);

    assert_eq!(fecha.dia, 1);
    assert_eq!(fecha.mes, 1);
    assert_eq!(fecha.año, 2025);
}

#[test]
fn test_restar_dias() {
    let mut fecha = Fecha::new(31, 12, 2024);
    fecha.restar_dias(365);

    assert_eq!(fecha.dia, 1);
    assert_eq!(fecha.mes, 1);
    assert_eq!(fecha.año, 2024);

    fecha.restar_dias(1);

    assert_eq!(fecha.dia, 31);
    assert_eq!(fecha.mes, 12);
    assert_eq!(fecha.año, 2023);
}