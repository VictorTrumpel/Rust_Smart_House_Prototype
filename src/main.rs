// Метка todo - реализовать самостоятельно

// ***** Пример библиотеки "Умный дом" со статическим содержимым
struct Room {
    title: String,
    devices: Vec<String>,
}

impl Room {
    fn new(title: String) -> Self {
        Room {
            title: String::from(title),
            devices: vec![String::from("device1"), String::from("device2")],
        }
    }
}

struct SmartHouse {
    title: String,
    rooms: Vec<Room>,
}

impl SmartHouse {
    fn new(title: String) -> Self {
        let room1 = Room::new(String::from("room1"));
        let room2 = Room::new(String::from("room2"));

        SmartHouse {
            title: String::from(title),
            rooms: vec![room1, room2],
        }
    }

    fn get_rooms(&self) -> [&str; 2] {
        let room1 = &self.rooms[0];
        let room2 = &self.rooms[1];

        [room1.title.as_str(), room2.title.as_str()]
    }

    fn devices(&self, room_title: &str) -> [&str; 2] {
        let rooms = &self.rooms;

        let mut room_idx = 0;
        for i in 0..rooms.len() {
            let room = &rooms[i];
            if room.title == room_title {
                break;
            }
            room_idx += 1;
        }

        let room_devices = &rooms[room_idx];

        let device1 = room_devices.devices[0].as_str();
        let device2 = room_devices.devices[1].as_str();

        [device1, device2]
    }

    fn create_report(&self, info_provider: &OwningDeviceInfoProvider) -> String {
        let socket = &info_provider.socket;
        let title = &socket.title;
        String::from(title)
    }
}

trait DeviceInfoProvider {
    // todo: метод, возвращающий состояние устройства по имени комнаты и имени устройства
}

// ***** Пример использования библиотеки умный дом:

// Пользовательские устройства:
struct SmartSocket {
    title: String,
}
struct SmartThermometer {
    title: String,
}

// Пользовательские поставщики информации об устройствах.
// Могут как хранить устройства, так и заимствывать.
struct OwningDeviceInfoProvider {
    socket: SmartSocket,
}
struct BorrowingDeviceInfoProvider<'a, 'b> {
    socket: &'a SmartSocket,
    thermo: &'b SmartThermometer,
}

// todo: реализация трейта `DeviceInfoProvider` для поставщиков информации

fn main() {
    // Инициализация устройств
    let socket1 = SmartSocket {
        title: String::from("socket1"),
    };
    let socket2 = SmartSocket {
        title: String::from("socket2"),
    };
    let thermo = SmartThermometer {
        title: String::from("term1"),
    };

    // Инициализация дома
    let house = SmartHouse::new(String::from("House 1"));

    // Строим отчёт с использованием `OwningDeviceInfoProvider`.
    let info_provider_1 = OwningDeviceInfoProvider { socket: socket1 };
    // todo: после добавления обобщённого аргумента в метод, расскоментировать передачу параметра
    let report1 = house.create_report(&info_provider_1);

    // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
    let _info_provider_2 = BorrowingDeviceInfoProvider {
        socket: &socket2,
        thermo: &thermo,
    };
    // todo: после добавления обобщённого аргумента в метод, расскоментировать передачу параметра
    // let report2 = house.create_report(/* &info_provider_2 */);

    // Выводим отчёты на экран:
    println!("Report #1: {report1}");
    // println!("Report #2: {report2}");
}
