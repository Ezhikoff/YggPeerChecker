use iced::widget::{
    button, column, container, pick_list, row, text, text_input, Column, Container, Scrollable,
    Space,
};
use iced::{Alignment, Background, Border, Color, Element, Length, Task, Theme};
use tokio::io::AsyncWriteExt;

// ── Модели ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
enum Protocol {
    Tcp,
    Tls,
    Quic,
    Ws,
    Wss,
    Socks,
    SocksTls,
    Unix,
}

impl Protocol {
    fn label(&self) -> &'static str {
        match self {
            Protocol::Tcp => "TCP",
            Protocol::Tls => "TLS",
            Protocol::Quic => "QUIC",
            Protocol::Ws => "WS",
            Protocol::Wss => "WSS",
            Protocol::Socks => "SOCKS",
            Protocol::SocksTls => "SOCKSTLS",
            Protocol::Unix => "UNIX",
        }
    }

    fn badge_color(&self) -> Color {
        match self {
            Protocol::Tcp => Color::from_rgb(0.2, 0.4, 0.7),
            Protocol::Tls => Color::from_rgb(0.1, 0.6, 0.3),
            Protocol::Quic => Color::from_rgb(0.5, 0.2, 0.7),
            Protocol::Ws => Color::from_rgb(0.7, 0.4, 0.1),
            Protocol::Wss => Color::from_rgb(0.1, 0.5, 0.7),
            Protocol::Socks => Color::from_rgb(0.5, 0.5, 0.1),
            Protocol::SocksTls => Color::from_rgb(0.3, 0.6, 0.6),
            Protocol::Unix => Color::from_rgb(0.4, 0.4, 0.4),
        }
    }
}

impl std::fmt::Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label())
    }
}

#[derive(Debug, Clone)]
struct Peer {
    protocol: Protocol,
    address: String,
    port: u16,
    raw: String,
}

impl Peer {
    fn parse(raw: &str) -> Option<Self> {
        let raw = raw.trim();
        if raw.is_empty() {
            return None;
        }

        let protocol = if let Some(addr) = raw.strip_prefix("tcp://") {
            (Protocol::Tcp, addr)
        } else if let Some(addr) = raw.strip_prefix("tls://") {
            (Protocol::Tls, addr)
        } else if let Some(addr) = raw.strip_prefix("quic://") {
            (Protocol::Quic, addr)
        } else if let Some(addr) = raw.strip_prefix("ws://") {
            (Protocol::Ws, addr)
        } else if let Some(addr) = raw.strip_prefix("wss://") {
            (Protocol::Wss, addr)
        } else if let Some(addr) = raw.strip_prefix("socks://") {
            (Protocol::Socks, addr)
        } else if let Some(addr) = raw.strip_prefix("sockstls://") {
            (Protocol::SocksTls, addr)
        } else if let Some(addr) = raw.strip_prefix("unix://") {
            (Protocol::Unix, addr)
        } else {
            return None;
        };

        let (protocol, rest) = protocol;

        // Обработка IPv6 в квадратных скобках
        let (address, port) = if let Some(start) = rest.find('[') {
            let end = rest.find(']')?;
            let addr = rest[start..=end].to_string();
            let port_str = &rest[end + 2..]; // после "]:"
            let port: u16 = port_str.parse().ok()?;
            (addr, port)
        } else {
            let parts: Vec<&str> = rest.rsplitn(2, ':').collect();
            if parts.len() != 2 {
                return None;
            }
            let port: u16 = parts[0].parse().ok()?;
            let address = parts[1].to_string();
            (address, port)
        };

        Some(Peer {
            protocol,
            address,
            port,
            raw: raw.to_string(),
        })
    }

    fn socket_addr(&self) -> Option<std::net::SocketAddr> {
        let addr_str = self.address.replace('[', "").replace(']', "");
        format!("{}:{}", addr_str, self.port)
            .parse()
            .ok()
    }
}

#[derive(Debug, Clone, PartialEq)]
enum PeerStatus {
    Pending,
    Checking,
    Online { ping_ms: u128, speed_mbps: f64 },
    Offline { error: String },
}

#[derive(Debug, Clone)]
struct PeerEntry {
    peer: Peer,
    status: PeerStatus,
}

#[derive(Debug, Clone, PartialEq)]
enum SortBy {
    Order,
    Ping,
    Speed,
    Status,
}

impl std::fmt::Display for SortBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortBy::Order => write!(f, "По порядку"),
            SortBy::Ping => write!(f, "По пингу"),
            SortBy::Speed => write!(f, "По скорости"),
            SortBy::Status => write!(f, "По статусу"),
        }
    }
}

// ── Стили текста ────────────────────────────────────────────────────────

fn text_style(color: Color) -> impl Fn(&Theme) -> text::Style {
    move |_| text::Style {
        color: Some(color),
    }
}

// ── Приложение ──────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    AddPeer,
    CheckAll,
    CheckDone(Vec<PeerEntry>),
    ClearResults,
    SortChanged(SortBy),
    RemovePeer(usize),
}

struct YggPeerChecker {
    input: String,
    peers: Vec<PeerEntry>,
    checking: bool,
    sort_by: SortBy,
}

impl Default for YggPeerChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl YggPeerChecker {
    fn new() -> Self {
        Self {
            input: String::new(),
            peers: Vec::new(),
            checking: false,
            sort_by: SortBy::Order,
        }
    }

    fn title(&self) -> String {
        String::from("YggPeerChecker - Проверка пиров Yggdrasil")
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::InputChanged(value) => {
                self.input = value;
                Task::none()
            }
            Message::AddPeer => {
                let lines: Vec<&str> = self.input.lines().collect();
                for line in lines {
                    if let Some(peer) = Peer::parse(line) {
                        self.peers.push(PeerEntry {
                            peer,
                            status: PeerStatus::Pending,
                        });
                    }
                }
                self.input.clear();
                Task::none()
            }
            Message::CheckAll => {
                if self.peers.is_empty() || self.checking {
                    return Task::none();
                }
                self.checking = true;
                for entry in &mut self.peers {
                    entry.status = PeerStatus::Checking;
                }
                let peers: Vec<Peer> = self.peers.iter().map(|e| e.peer.clone()).collect();

                Task::perform(check_peers(peers), Message::CheckDone)
            }
            Message::CheckDone(entries) => {
                self.peers = entries;
                self.checking = false;
                Task::none()
            }
            Message::ClearResults => {
                for entry in &mut self.peers {
                    entry.status = PeerStatus::Pending;
                }
                Task::none()
            }
            Message::SortChanged(sort_by) => {
                self.sort_by = sort_by;
                Task::none()
            }
            Message::RemovePeer(index) => {
                self.peers.remove(index);
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let title = text("YggPeerChecker")
            .size(28)
            .style(text_style(Color::from_rgb(0.0, 0.8, 0.0)));

        let subtitle = text("Проверка пиров Yggdrasil Network")
            .size(14)
            .style(text_style(Color::from_rgb(0.6, 0.6, 0.6)));

        let header = column![title, subtitle].spacing(4);

        // Поле ввода
        let input_area = column![
            text("Добавить пиры (по одному на строку):").size(14),
            text_input(
                "tcp://89.44.86.85:65535\nquic://[2a09:5302:ffff::132a]:65535",
                &self.input
            )
            .on_input(Message::InputChanged)
            .on_submit(Message::AddPeer)
            .size(14)
            .padding(8),
            row![
                button(text("Добавить").size(13))
                    .padding([6, 16])
                    .on_press(Message::AddPeer),
                button(if self.checking {
                    text("Проверка...").size(13)
                } else {
                    text("Проверить все").size(13)
                })
                .padding([6, 16])
                .on_press_maybe(if !self.checking && !self.peers.is_empty() {
                    Some(Message::CheckAll)
                } else {
                    None
                }),
                button(text("Сбросить").size(13))
                    .padding([6, 16])
                    .on_press(Message::ClearResults),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
        ]
        .spacing(8);

        // Сортировка
        let sort_options = vec![SortBy::Order, SortBy::Ping, SortBy::Speed, SortBy::Status];
        let sort_row = row![
            text("Сортировка:").size(13),
            pick_list(sort_options, Some(&self.sort_by), Message::SortChanged).width(Length::Fixed(130.0)),
        ]
        .spacing(8)
        .align_y(Alignment::Center);

        // Список пиров
        let peers_list: Container<Message> = if self.peers.is_empty() {
            container(
                text("Нет пиров. Добавьте пиры для проверки.")
                    .size(14)
                    .style(text_style(Color::from_rgb(0.5, 0.5, 0.5))),
            )
            .padding(16)
        } else {
            let mut sorted_peers: Vec<(usize, &PeerEntry)> =
                self.peers.iter().enumerate().collect();

            match self.sort_by {
                SortBy::Ping => {
                    sorted_peers.sort_by(|a, b| {
                        let ping_a = match &a.1.status {
                            PeerStatus::Online { ping_ms, .. } => *ping_ms,
                            _ => u128::MAX,
                        };
                        let ping_b = match &b.1.status {
                            PeerStatus::Online { ping_ms, .. } => *ping_ms,
                            _ => u128::MAX,
                        };
                        ping_a.cmp(&ping_b)
                    });
                }
                SortBy::Speed => {
                    sorted_peers.sort_by(|a, b| {
                        let speed_a = match &a.1.status {
                            PeerStatus::Online { speed_mbps, .. } => speed_mbps.to_bits(),
                            _ => 0,
                        };
                        let speed_b = match &b.1.status {
                            PeerStatus::Online { speed_mbps, .. } => speed_mbps.to_bits(),
                            _ => 0,
                        };
                        speed_b.cmp(&speed_a)
                    });
                }
                SortBy::Status => {
                    sorted_peers.sort_by(|a, b| {
                        let status_order = |e: &PeerEntry| -> u8 {
                            match &e.status {
                                PeerStatus::Online { .. } => 0,
                                PeerStatus::Pending => 1,
                                PeerStatus::Checking => 2,
                                PeerStatus::Offline { .. } => 3,
                            }
                        };
                        status_order(a.1).cmp(&status_order(b.1))
                    });
                }
                SortBy::Order => {}
            }

            let mut peer_column: Column<Message> = column![];
            peer_column = peer_column.spacing(4);

            for (orig_idx, entry) in sorted_peers {
                peer_column = peer_column.push(peer_row(entry, orig_idx));
            }

            container(Scrollable::new(peer_column).height(Length::Fixed(400.0))).padding(8)
        };

        // Статистика
        let online = self
            .peers
            .iter()
            .filter(|e| matches!(e.status, PeerStatus::Online { .. }))
            .count();
        let offline = self
            .peers
            .iter()
            .filter(|e| matches!(e.status, PeerStatus::Offline { .. }))
            .count();
        let pending = self.peers.len() - online - offline;

        let stats = row![
            status_badge("Онлайн", Color::from_rgb(0.0, 0.7, 0.0), online),
            status_badge("Офлайн", Color::from_rgb(0.8, 0.0, 0.0), offline),
            status_badge("Ожидание", Color::from_rgb(0.5, 0.5, 0.5), pending),
        ]
        .spacing(12);

        let content = column![
            header,
            Space::with_height(16),
            input_area,
            Space::with_height(12),
            sort_row,
            Space::with_height(8),
            peers_list,
            Space::with_height(8),
            stats,
        ]
        .spacing(8)
        .padding(20);

        container(content).into()
    }
}

fn status_badge(label: &str, color: Color, count: usize) -> Container<'_, Message> {
    let badge = row![
        text("").width(Length::Fixed(10.0)).style(text_style(color)),
        text(format!("{}: {}", label, count)).size(13),
    ]
    .spacing(6)
    .align_y(Alignment::Center);

    container(badge).padding([4, 10]).style(move |_theme: &Theme| container::Style {
        border: Border {
            color,
            width: 1.0,
            radius: 4.0.into(),
        },
        ..Default::default()
    })
}

fn peer_row(entry: &PeerEntry, index: usize) -> Element<'_, Message> {
    let status_color = match &entry.status {
        PeerStatus::Online { ping_ms, .. } => {
            if *ping_ms < 100 {
                Color::from_rgb(0.0, 0.8, 0.0) // Ярко-зелёный — отличный
            } else if *ping_ms < 300 {
                Color::from_rgb(0.5, 0.8, 0.0) // Жёлто-зелёный — хороший
            } else {
                Color::from_rgb(0.8, 0.8, 0.0) // Жёлтый — приёмлемый
            }
        }
        PeerStatus::Offline { .. } => Color::from_rgb(0.8, 0.0, 0.0), // Красный — офлайн
        PeerStatus::Checking => Color::from_rgb(0.8, 0.6, 0.0),       // Оранжевый — проверка
        PeerStatus::Pending => Color::from_rgb(0.5, 0.5, 0.5),        // Серый — ожидание
    };

    let status_text = match &entry.status {
        PeerStatus::Online { ping_ms, speed_mbps } => {
            format!("Пинг: {}мс | Скорость: {:.2} Мбит/с", ping_ms, speed_mbps)
        }
        PeerStatus::Offline { error } => format!("Ошибка: {}", error),
        PeerStatus::Checking => "Проверка...".to_string(),
        PeerStatus::Pending => "Ожидание".to_string(),
    };

    let protocol_badge = container(
        text(entry.peer.protocol.label())
            .size(11)
            .style(text_style(Color::WHITE)),
    )
    .padding([2, 6])
    .style(move |_theme: &Theme| container::Style {
        background: Some(Background::Color(entry.peer.protocol.badge_color())),
        border: Border {
            radius: 4.0.into(),
            ..Default::default()
        },
        ..Default::default()
    });

    let addr_text = text(&entry.peer.raw).size(13);
    let status_text = text(status_text).size(12).style(text_style(status_color));

    let remove_btn = button(text("✕").size(12))
        .padding([2, 8])
        .on_press(Message::RemovePeer(index));

    let row_widget = row![
        protocol_badge,
        addr_text.width(Length::Fill),
        status_text.width(Length::Fixed(300.0)),
        remove_btn,
    ]
    .spacing(8)
    .align_y(Alignment::Center)
    .padding([6, 8]);

    container(row_widget)
        .style(move |_theme: &Theme| container::Style {
            background: Some(Background::Color(Color {
                r: status_color.r * 0.15,
                g: status_color.g * 0.15,
                b: status_color.b * 0.15,
                a: 0.3,
            })),
            border: Border {
                color: status_color,
                width: 1.0,
                radius: 6.0.into(),
            },
            ..Default::default()
        })
        .into()
}

// ── Логика проверки пиров ──────────────────────────────────────────────

async fn check_peers(peers: Vec<Peer>) -> Vec<PeerEntry> {
    let mut results = Vec::new();

    for peer in peers {
        let status = check_peer(&peer).await;
        results.push(PeerEntry {
            peer,
            status,
        });
    }

    results
}

async fn check_peer(peer: &Peer) -> PeerStatus {
    let socket_addr = match peer.socket_addr() {
        Some(addr) => addr,
        None => return PeerStatus::Offline { error: "Неверный адрес".to_string() },
    };

    match peer.protocol {
        Protocol::Tcp => check_tcp(socket_addr).await,
        Protocol::Tls => check_tls(socket_addr).await,
        Protocol::Quic => check_quic(socket_addr).await,
        Protocol::Ws => check_ws(socket_addr, false).await,
        Protocol::Wss => check_ws(socket_addr, true).await,
        Protocol::Socks => check_socks(&peer, false).await,
        Protocol::SocksTls => check_socks(&peer, true).await,
        Protocol::Unix => PeerStatus::Offline {
            error: "UNIX сокеты не поддерживаются на Windows".to_string(),
        },
    }
}

async fn check_tcp(socket_addr: std::net::SocketAddr) -> PeerStatus {
    let start = std::time::Instant::now();

    let timeout = tokio::time::Duration::from_secs(3);
    let result = tokio::time::timeout(timeout, tokio::net::TcpStream::connect(socket_addr)).await;

    let ping_ms = start.elapsed().as_millis();

    match result {
        Ok(Ok(_stream)) => {
            let speed = measure_speed_tcp(socket_addr).await;
            PeerStatus::Online {
                ping_ms,
                speed_mbps: speed,
            }
        }
        Ok(Err(e)) => PeerStatus::Offline {
            error: e.to_string(),
        },
        Err(_) => PeerStatus::Offline {
            error: "Таймаут подключения".to_string(),
        },
    }
}

async fn measure_speed_tcp(socket_addr: std::net::SocketAddr) -> f64 {
    let start = std::time::Instant::now();
    let mut total_bytes = 0u64;

    if let Ok(Ok(mut stream)) =
        tokio::time::timeout(tokio::time::Duration::from_secs(1), tokio::net::TcpStream::connect(socket_addr)).await
    {
        let test_data = vec![0u8; 8192];

        while start.elapsed().as_secs_f64() < 1.0 {
            match tokio::time::timeout(
                tokio::time::Duration::from_millis(500),
                stream.write_all(&test_data),
            )
            .await
            {
                Ok(Ok(_)) => total_bytes += test_data.len() as u64,
                _ => break,
            }
        }
    }

    let elapsed = start.elapsed().as_secs_f64();
    if elapsed > 0.0 {
        (total_bytes as f64 * 8.0) / elapsed / 1_000_000.0
    } else {
        0.0
    }
}

// ── TLS проверка ──────────────────────────────────────────────────────

async fn check_tls(socket_addr: std::net::SocketAddr) -> PeerStatus {
    use tokio_rustls::rustls::{ClientConfig, RootCertStore};
    use std::sync::Arc;

    let start = std::time::Instant::now();

    let mut root_store = RootCertStore::empty();
    root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());

    let config = ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    let connector = tokio_rustls::TlsConnector::from(Arc::new(config));

    let timeout = tokio::time::Duration::from_secs(3);
    let tcp_result = tokio::time::timeout(timeout, tokio::net::TcpStream::connect(socket_addr)).await;

    let tcp_stream = match tcp_result {
        Ok(Ok(s)) => s,
        Ok(Err(e)) => return PeerStatus::Offline { error: e.to_string() },
        Err(_) => return PeerStatus::Offline { error: "Таймаут TCP".to_string() },
    };

    let domain = socket_addr.ip().to_string();
    let dns_name = match rustls::pki_types::ServerName::try_from(domain) {
        Ok(n) => n,
        Err(_) => return PeerStatus::Offline { error: "Неверное имя сервера".to_string() },
    };

    let tls_result = tokio::time::timeout(timeout, connector.connect(dns_name, tcp_stream)).await;

    let ping_ms = start.elapsed().as_millis();

    match tls_result {
        Ok(Ok(_stream)) => PeerStatus::Online {
            ping_ms,
            speed_mbps: 0.0,
        },
        Ok(Err(e)) => PeerStatus::Offline { error: format!("TLS ошибка: {}", e) },
        Err(_) => PeerStatus::Offline { error: "Таймаут TLS handshake".to_string() },
    }
}

// ── WebSocket / WSS проверка ──────────────────────────────────────────

async fn check_ws(socket_addr: std::net::SocketAddr, use_tls: bool) -> PeerStatus {
    use tokio_tungstenite::tungstenite::client::IntoClientRequest;
    use tokio_tungstenite::Connector;

    let start = std::time::Instant::now();
    let scheme = if use_tls { "wss" } else { "ws" };
    let url = format!("{}://{}/", scheme, socket_addr);

    let request = match url.clone().into_client_request() {
        Ok(r) => r,
        Err(e) => return PeerStatus::Offline { error: format!("Ошибка URL: {}", e) },
    };

    let timeout = tokio::time::Duration::from_secs(3);

    if use_tls {
        use tokio_rustls::rustls::{ClientConfig, RootCertStore};
        use std::sync::Arc;

        let mut root_store = RootCertStore::empty();
        root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());

        let tls_config = ClientConfig::builder()
            .dangerous()
            .with_custom_certificate_verifier(Arc::new(SkipServerVerification))
            .with_no_client_auth();

        let connector = Connector::Rustls(Arc::new(tls_config));

        let connect_fut = tokio_tungstenite::connect_async_tls_with_config(request, None, false, Some(connector));
        let result = tokio::time::timeout(timeout, connect_fut).await;

        let ping_ms = start.elapsed().as_millis();

        match result {
            Ok(Ok((_, _response))) => PeerStatus::Online { ping_ms, speed_mbps: 0.0 },
            Ok(Err(e)) => PeerStatus::Offline { error: format!("WS ошибка: {}", e) },
            Err(_) => PeerStatus::Offline { error: "Таймаут WS подключения".to_string() },
        }
    } else {
        let connect_fut = tokio_tungstenite::connect_async(request);
        let result = tokio::time::timeout(timeout, connect_fut).await;

        let ping_ms = start.elapsed().as_millis();

        match result {
            Ok(Ok((_, _response))) => PeerStatus::Online { ping_ms, speed_mbps: 0.0 },
            Ok(Err(e)) => PeerStatus::Offline { error: format!("WS ошибка: {}", e) },
            Err(_) => PeerStatus::Offline { error: "Таймаут WS подключения".to_string() },
        }
    }
}

// ── SOCKS / SOCKS+TLS проверка ───────────────────────────────────────

async fn check_socks(_peer: &Peer, _use_tls: bool) -> PeerStatus {
    // SOCKS требует прокси-сервера и отдельной логики перенаправления
    // Для Yggdrasil пиров это менее актуально
    PeerStatus::Offline {
        error: "SOCKS прокси не реализован".to_string(),
    }
}

async fn check_quic(socket_addr: std::net::SocketAddr) -> PeerStatus {
    let start = std::time::Instant::now();
    let server_name = "example.com";

    // Извлекаем endpoint до await, чтобы избежать проблемы Send
    let endpoint = match make_quic_client_config() {
        Ok((_, ep)) => ep,
        Err(e) => {
            return PeerStatus::Offline {
                error: format!("Ошибка TLS конфигурации: {}", e),
            }
        }
    };

    let connecting = match endpoint.connect(socket_addr, server_name) {
        Ok(c) => c,
        Err(e) => {
            return PeerStatus::Offline {
                error: format!("QUIC connect error: {}", e),
            }
        }
    };

    let timeout = tokio::time::Duration::from_secs(3);
    let result = tokio::time::timeout(timeout, connecting).await;

    let ping_ms = start.elapsed().as_millis();

    match result {
        Ok(Ok(_connection)) => {
            PeerStatus::Online {
                ping_ms,
                speed_mbps: 0.0,
            }
        }
        Ok(Err(e)) => PeerStatus::Offline {
            error: format!("QUIC ошибка: {}", e),
        },
        Err(_) => PeerStatus::Offline {
            error: "Таймаут QUIC подключения".to_string(),
        },
    }
}

fn make_quic_client_config(
) -> Result<(quinn::ClientConfig, quinn::Endpoint), Box<dyn std::error::Error>> {
    use std::sync::Arc;

    let rcgen::CertifiedKey { cert, key_pair } =
        rcgen::generate_simple_self_signed(vec!["example.com".to_string()])?;

    let client_cert: rustls::pki_types::CertificateDer =
        cert.der().as_ref().to_vec().into();
    let priv_key = key_pair.serialize_der();
    let client_key = rustls::pki_types::PrivateKeyDer::try_from(priv_key)?;

    let mut root_store = rustls::RootCertStore::empty();
    root_store.add(client_cert.clone())?;

    let crypto = rustls::ClientConfig::builder()
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(SkipServerVerification))
        .with_client_auth_cert(vec![client_cert], client_key)?;

    let config = quinn::ClientConfig::new(Arc::new(
        quinn::crypto::rustls::QuicClientConfig::try_from(crypto)?,
    ));

    let mut endpoint = quinn::Endpoint::client("[::]:0".parse().unwrap())?;
    endpoint.set_default_client_config(config.clone());

    Ok((config, endpoint))
}

#[derive(Debug)]
struct SkipServerVerification;

impl rustls::client::danger::ServerCertVerifier for SkipServerVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &rustls::pki_types::CertificateDer<'_>,
        _intermediates: &[rustls::pki_types::CertificateDer<'_>],
        _server_name: &rustls::pki_types::ServerName<'_>,
        _ocsp_response: &[u8],
        _now: rustls::pki_types::UnixTime,
    ) -> Result<rustls::client::danger::ServerCertVerified, rustls::Error> {
        Ok(rustls::client::danger::ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        _message: &[u8],
        _cert: &rustls::pki_types::CertificateDer<'_>,
        _dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        Ok(rustls::client::danger::HandshakeSignatureValid::assertion())
    }

    fn verify_tls13_signature(
        &self,
        _message: &[u8],
        _cert: &rustls::pki_types::CertificateDer<'_>,
        _dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        Ok(rustls::client::danger::HandshakeSignatureValid::assertion())
    }

    fn supported_verify_schemes(&self) -> Vec<rustls::SignatureScheme> {
        vec![
            rustls::SignatureScheme::RSA_PKCS1_SHA256,
            rustls::SignatureScheme::RSA_PKCS1_SHA384,
            rustls::SignatureScheme::RSA_PKCS1_SHA512,
            rustls::SignatureScheme::ECDSA_NISTP256_SHA256,
            rustls::SignatureScheme::ECDSA_NISTP384_SHA384,
            rustls::SignatureScheme::ECDSA_NISTP521_SHA512,
            rustls::SignatureScheme::ED25519,
        ]
    }
}

// ── Точка входа ─────────────────────────────────────────────────────────

fn main() -> iced::Result {
    iced::application(YggPeerChecker::title, YggPeerChecker::update, YggPeerChecker::view)
        .run()
}
