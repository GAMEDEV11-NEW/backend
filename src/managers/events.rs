use socketioxide::extract::{Data, SocketRef};
use socketioxide::SocketIo;
use serde_json::json;
use tracing::{info, warn, error};
use rand::Rng;
use std::sync::Arc;
use bson::to_document;

use crate::managers::connection::ConnectionManager;
use crate::managers::validation::ValidationManager;
use crate::managers::jwt::create_jwt_service;
use crate::database::service::DataService;

// Localized success messages structure
#[derive(Debug, Clone)]
struct LocalizedMessages {
    welcome_message: String,
    setup_complete: String,
    ready_to_play: String,
    next_steps: String,
}

// Function to get localized success messages based on language code
fn get_localized_success_messages(language_code: &str) -> LocalizedMessages {
    match language_code {
        "en" => LocalizedMessages {
            welcome_message: "Welcome to Game Admin! 🎮".to_string(),
            setup_complete: "Setup completed successfully! ✅".to_string(),
            ready_to_play: "You're all set to start gaming! 🚀".to_string(),
            next_steps: "Explore the dashboard and start managing your game experience.".to_string(),
        },
        "es" => LocalizedMessages {
            welcome_message: "¡Bienvenido a Game Admin! 🎮".to_string(),
            setup_complete: "¡Configuración completada exitosamente! ✅".to_string(),
            ready_to_play: "¡Estás listo para comenzar a jugar! 🚀".to_string(),
            next_steps: "Explora el panel y comienza a gestionar tu experiencia de juego.".to_string(),
        },
        "fr" => LocalizedMessages {
            welcome_message: "Bienvenue sur Game Admin ! 🎮".to_string(),
            setup_complete: "Configuration terminée avec succès ! ✅".to_string(),
            ready_to_play: "Vous êtes prêt à commencer à jouer ! 🚀".to_string(),
            next_steps: "Explorez le tableau de bord et commencez à gérer votre expérience de jeu.".to_string(),
        },
        "de" => LocalizedMessages {
            welcome_message: "Willkommen bei Game Admin! 🎮".to_string(),
            setup_complete: "Setup erfolgreich abgeschlossen! ✅".to_string(),
            ready_to_play: "Du bist bereit zum Spielen! 🚀".to_string(),
            next_steps: "Erkunde das Dashboard und beginne mit der Verwaltung deines Spielerlebnisses.".to_string(),
        },
        "hi" => LocalizedMessages {
            welcome_message: "Game Admin में आपका स्वागत है! 🎮".to_string(),
            setup_complete: "सेटअप सफलतापूर्वक पूरा हुआ! ✅".to_string(),
            ready_to_play: "आप गेमिंग शुरू करने के लिए तैयार हैं! 🚀".to_string(),
            next_steps: "डैशबोर्ड का अन्वेषण करें और अपने गेमिंग अनुभव का प्रबंधन शुरू करें।".to_string(),
        },
        "zh" => LocalizedMessages {
            welcome_message: "欢迎来到游戏管理！🎮".to_string(),
            setup_complete: "设置成功完成！✅".to_string(),
            ready_to_play: "您已准备好开始游戏！🚀".to_string(),
            next_steps: "探索仪表板并开始管理您的游戏体验。".to_string(),
        },
        "ja" => LocalizedMessages {
            welcome_message: "Game Adminへようこそ！🎮".to_string(),
            setup_complete: "セットアップが正常に完了しました！✅".to_string(),
            ready_to_play: "ゲームを始める準備ができました！🚀".to_string(),
            next_steps: "ダッシュボードを探索し、ゲーム体験の管理を開始してください。".to_string(),
        },
        "ko" => LocalizedMessages {
            welcome_message: "Game Admin에 오신 것을 환영합니다! 🎮".to_string(),
            setup_complete: "설정이 성공적으로 완료되었습니다! ✅".to_string(),
            ready_to_play: "게임을 시작할 준비가 되었습니다! 🚀".to_string(),
            next_steps: "대시보드를 탐색하고 게임 경험 관리를 시작하세요.".to_string(),
        },
        "ar" => LocalizedMessages {
            welcome_message: "مرحباً بك في إدارة الألعاب! 🎮".to_string(),
            setup_complete: "تم إكمال الإعداد بنجاح! ✅".to_string(),
            ready_to_play: "أنت جاهز لبدء اللعب! 🚀".to_string(),
            next_steps: "استكشف لوحة التحكم وابدأ في إدارة تجربة اللعب الخاصة بك.".to_string(),
        },
        "pt" => LocalizedMessages {
            welcome_message: "Bem-vindo ao Game Admin! 🎮".to_string(),
            setup_complete: "Configuração concluída com sucesso! ✅".to_string(),
            ready_to_play: "Você está pronto para começar a jogar! 🚀".to_string(),
            next_steps: "Explore o painel e comece a gerenciar sua experiência de jogo.".to_string(),
        },
        "ru" => LocalizedMessages {
            welcome_message: "Добро пожаловать в Game Admin! 🎮".to_string(),
            setup_complete: "Настройка успешно завершена! ✅".to_string(),
            ready_to_play: "Вы готовы начать играть! 🚀".to_string(),
            next_steps: "Исследуйте панель управления и начните управлять своим игровым опытом.".to_string(),
        },
        _ => LocalizedMessages {
            welcome_message: "Welcome to Game Admin! 🎮".to_string(),
            setup_complete: "Setup completed successfully! ✅".to_string(),
            ready_to_play: "You're all set to start gaming! 🚀".to_string(),
            next_steps: "Explore the dashboard and start managing your game experience.".to_string(),
        },
    }
}

pub struct EventManager;

impl EventManager {
    pub fn register_custom_events(io: &SocketIo, data_service: Arc<DataService>) {
        io.ns("/", move |socket: SocketRef| {
            let data_service = data_service.clone();
            async move {
                info!("🔌 New client connected: {}", socket.id);
                ConnectionManager::send_connect_response(&socket, data_service.clone()).await;

                // Handle device info event
                let ds1 = data_service.clone();
                socket.on("device:info", move |socket: SocketRef, Data::<serde_json::Value>(data)| {
                    let ds1 = ds1.clone();
                    async move {
                        info!("📱 Received device info from {}: {:?}", socket.id, data);
                        let _ = ds1.store_device_info_event(&socket.id.to_string(), &data).await;
                        match ValidationManager::validate_device_info(&data) {
                            Ok(_) => {
                                let ack_response = json!({
                                    "status": "success",
                                    "message": "Device info received and validated",
                                    "timestamp": chrono::Utc::now().to_rfc3339(),
                                    "socket_id": socket.id.to_string(),
                                    "event": "device:info:ack"
                                });
                                match socket.emit("device:info:ack", ack_response) {
                                    Ok(_) => info!("Sent device info acknowledgment to: {}", socket.id),
                                    Err(e) => warn!("⚠️ Failed to emit device:info:ack for socket {}: {}", socket.id, e),
                                }
                            }
                            Err(error_details) => {
                                let error_response = json!({
                                    "status": "error",
                                    "error_code": error_details.code,
                                    "error_type": error_details.error_type,
                                    "field": error_details.field,
                                    "message": error_details.message,
                                    "details": error_details.details,
                                    "timestamp": chrono::Utc::now().to_rfc3339(),
                                    "socket_id": socket.id.to_string(),
                                    "event": "connection_error"
                                });
                                let payload_doc = to_document(&error_response).unwrap_or_default();
                                let _ = ds1.store_connection_error_event(
                                    &socket.id.to_string(),
                                    &error_details.code,
                                    &error_details.error_type,
                                    &error_details.field,
                                    &error_details.message,
                                    payload_doc
                                ).await;
                                let _ = socket.emit("connection_error", error_response);
                                info!("Sent connection error to {}: {:?}", socket.id, error_details);
                            }
                        }
                    }
                });

                // Handle login event
                let ds2 = data_service.clone();
                socket.on("login", move |socket: SocketRef, Data::<serde_json::Value>(data)| {
                    let ds2 = ds2.clone();
                    async move {
                        tracing::info!("🔐 [DEBUG] Login event handler triggered");
                        info!("🔐 Received login request from {}: {:?}", socket.id, data);
                        let mobile_no = data["mobile_no"].as_str().unwrap_or("unknown");
                        let device_id = data["device_id"].as_str().unwrap_or("unknown");
                        let fcm_token = data["fcm_token"].as_str().unwrap_or("unknown");
                        let email = data["email"].as_str();
                        let _ = ds2.store_login_event(&socket.id.to_string(), mobile_no, device_id, fcm_token, email).await;
                        match ValidationManager::validate_login_data(&data) {
                            Ok(_) => {
                                let mobile_no = data["mobile_no"].as_str().unwrap_or("unknown");
                                let device_id = data["device_id"].as_str().unwrap_or("unknown");
                                let session_token = rand::thread_rng().gen_range(100000000..999999999).to_string();
                                let otp = rand::thread_rng().gen_range(100000..999999);
                                
                                // Check if user exists in userregister collection
                                let user_exists = ds2.user_exists(mobile_no).await;
                                let is_new_user = match user_exists {
                                    Ok(exists) => {
                                        if exists {
                                            // User exists - update login info
                                            let update_result = ds2.update_user_login_info(mobile_no).await;
                                            if let Err(e) = update_result {
                                                warn!("Failed to update user login info: {}", e);
                                            }
                                            info!("🔄 Existing user logged in: {}", mobile_no);
                                            false
                                        } else {
                                            // New user - register them
                                            let register_result = ds2.register_new_user(mobile_no, device_id, fcm_token, email).await;
                                            match register_result {
                                                Ok(_) => {
                                                    info!("🆕 New user registered: {}", mobile_no);
                                                }
                                                Err(e) => {
                                                    warn!("Failed to register new user: {}", e);
                                                }
                                            }
                                            true
                                        }
                                    }
                                    Err(e) => {
                                        warn!("Failed to check user existence: {}", e);
                                        false
                                    }
                                };
                                
                                let login_response = json!({
                                    "status": "success",
                                    "message": "Login successful",
                                    "mobile_no": mobile_no,
                                    "device_id": device_id,
                                    "session_token": session_token,
                                    "otp": otp,
                                    "is_new_user": is_new_user,
                                    "timestamp": chrono::Utc::now().to_rfc3339(),
                                    "socket_id": socket.id.to_string(),
                                    "event": "login:success"
                                });
                                let store_result = ds2.store_login_success_event(&socket.id.to_string(), mobile_no, device_id, &session_token, otp).await;
                                if let Err(e) = store_result {
                                    warn!("Failed to store login success event: {}", e);
                                }
                                // Add error handling for emit
                                match socket.emit("login:success", login_response) {
                                    Ok(_) => info!("✅ Login successful for mobile: {} (device: {}, socket: {})", mobile_no, device_id, socket.id),
                                    Err(e) => warn!("⚠️ Failed to emit login:success for mobile: {} (socket: {}): {}", mobile_no, socket.id, e),
                                }
                            }
                            Err(error_details) => {
                                let error_response = json!({
                                    "status": "error",
                                    "error_code": error_details.code,
                                    "error_type": error_details.error_type,
                                    "field": error_details.field,
                                    "message": error_details.message,
                                    "details": error_details.details,
                                    "timestamp": chrono::Utc::now().to_rfc3339(),
                                    "socket_id": socket.id.to_string(),
                                    "event": "connection_error"
                                });
                                let payload_doc = to_document(&error_response).unwrap_or_default();
                                let _ = ds2.store_connection_error_event(
                                    &socket.id.to_string(),
                                    &error_details.code,
                                    &error_details.error_type,
                                    &error_details.field,
                                    &error_details.message,
                                    payload_doc
                                ).await;
                                let _ = socket.emit("connection_error", error_response);
                                info!("❌ Login failed for socket {}: {:?}", socket.id, error_details);
                            }
                        }
                    }
                });

                // Handle OTP verification event
                let ds3 = data_service.clone();
                socket.on("verify:otp", move |socket: SocketRef, Data::<serde_json::Value>(data)| {
                    let ds3 = ds3.clone();
                    async move {
                        info!("🔢 Received OTP verification request from {}: {:?}", socket.id, data);
                        
                        match ValidationManager::validate_otp_data(&data) {
                            Ok(_) => {
                                let mobile_no = data["mobile_no"].as_str().unwrap_or("unknown");
                                let otp = data["otp"].as_str().unwrap_or("unknown");
                                let session_token = data["session_token"].as_str().unwrap_or("unknown");
                                
                                // Check rate limiting before verification
                                let rate_limit_check = ds3.check_otp_attempts(mobile_no, session_token).await;
                                match rate_limit_check {
                                    Ok(is_allowed) => {
                                        if !is_allowed {
                                            let error_response = json!({
                                                "status": "error",
                                                "error_code": "RATE_LIMIT_EXCEEDED",
                                                "error_type": "AUTHENTICATION_ERROR",
                                                "field": "otp",
                                                "message": "Too many OTP verification attempts. Please try again later.",
                                                "details": json!({
                                                    "mobile_no": mobile_no,
                                                    "session_token": session_token,
                                                    "max_attempts": 5
                                                }),
                                                "timestamp": chrono::Utc::now().to_rfc3339(),
                                                "socket_id": socket.id.to_string(),
                                                "event": "otp:verification_failed"
                                            });
                                            
                                            let payload_doc = to_document(&error_response).unwrap_or_default();
                                            let _ = ds3.store_connection_error_event(
                                                &socket.id.to_string(),
                                                "RATE_LIMIT_EXCEEDED",
                                                "AUTHENTICATION_ERROR",
                                                "otp",
                                                "Too many OTP verification attempts. Please try again later.",
                                                payload_doc
                                            ).await;
                                            
                                            let _ = socket.emit("otp:verification_failed", error_response);
                                            info!("🚫 Rate limit exceeded for mobile: {} (socket: {})", mobile_no, socket.id);
                                            return;
                                        }
                                    }
                                    Err(e) => {
                                        warn!("⚠️ Failed to check rate limit for mobile: {} (socket: {}): {}", mobile_no, socket.id, e);
                                        // Continue with verification if rate limit check fails
                                    }
                                }
                                
                                // Verify the OTP
                                let verify_result = ds3.verify_otp(&socket.id.to_string(), mobile_no, session_token, otp).await;
                                match verify_result {
                                    Ok(verification_result) => {
                                        match verification_result {
                                            crate::database::models::OtpVerificationResult::Success => {
                                                // Get user info
                                                let user_info = ds3.get_user_by_mobile(mobile_no).await;
                                                let (user_id, user_number) = match user_info {
                                                    Ok(Some(user)) => (user.user_id.clone(), user.user_number),
                                                    _ => {
                                                        // User not found, create new user
                                                        let (new_user_id, new_user_number) = ds3.register_new_user(
                                                            mobile_no,
                                                            data["device_id"].as_str().unwrap_or("unknown"),
                                                            data["fcm_token"].as_str().unwrap_or("unknown"),
                                                            data["email"].as_str()
                                                        ).await.unwrap_or(("unknown".to_string(), 0));
                                                        (new_user_id, new_user_number)
                                                    }
                                                };

                                                // Generate JWT token
                                                let jwt_service = create_jwt_service();
                                                let jwt_token = match jwt_service.generate_token(
                                                    &user_id,
                                                    user_number,
                                                    mobile_no,
                                                    data["device_id"].as_str().unwrap_or("unknown"),
                                                    data["fcm_token"].as_str().unwrap_or("unknown"),
                                                ) {
                                                    Ok(token) => token,
                                                    Err(e) => {
                                                        error!("❌ Failed to generate JWT token: {}", e);
                                                        "".to_string()
                                                    }
                                                };

                                                // Check if user is new or old by checking if a profile has been set
                                                let user_status = match ds3.get_user_by_mobile(mobile_no).await {
                                                    Ok(Some(user)) => {
                                                        if user.full_name.is_some() {
                                                            "existing_user"
                                                        } else {
                                                            "new_user"
                                                        }
                                                    }
                                                    _ => "new_user", // Default to new_user if lookup fails, though it shouldn't
                                                };

                                                let success_response = json!({
                                                    "status": "success",
                                                    "message": "OTP verification successful. Authentication completed.",
                                                    "mobile_no": mobile_no,
                                                    "session_token": session_token,
                                                    "user_id": user_id,
                                                    "user_number": user_number,
                                                    "user_status": user_status,
                                                    "jwt_token": jwt_token,
                                                    "token_type": "Bearer",
                                                    "expires_in": 604800, // 7 days in seconds
                                                    "timestamp": chrono::Utc::now().to_rfc3339(),
                                                    "socket_id": socket.id.to_string(),
                                                    "event": "otp:verified"
                                                });

                                                // Store OTP verification event with JWT token
                                                let _ = ds3.store_otp_verification_event(
                                                    &socket.id.to_string(),
                                                    mobile_no,
                                                    session_token,
                                                    otp,
                                                    true,
                                                    Some(&user_id),
                                                    Some(user_number),
                                                    Some(&jwt_token)
                                                ).await;

                                                // Store user registration event if new user
                                                if user_status == "new_user" {
                                                    let _ = ds3.store_user_registration_event(
                                                        &socket.id.to_string(),
                                                        &user_id,
                                                        user_number,
                                                        mobile_no,
                                                        data["device_id"].as_str().unwrap_or("unknown"),
                                                        data["fcm_token"].as_str().unwrap_or("unknown"),
                                                        data["email"].as_str()
                                                    ).await;
                                                }

                                                // Add error handling for emit
                                                match socket.emit("otp:verified", success_response) {
                                                    Ok(_) => info!("✅ OTP verification successful for mobile: {} (socket: {}, status: {}, user_id: {}, user_number: {})", mobile_no, socket.id, user_status, user_id, user_number),
                                                    Err(e) => warn!("⚠️ Failed to emit otp:verified for mobile: {} (socket: {}): {}", mobile_no, socket.id, e),
                                                }
                                            }
                                            crate::database::models::OtpVerificationResult::Invalid => {
                                                let error_response = json!({
                                                    "status": "error",
                                                    "error_code": "INVALID_OTP",
                                                    "error_type": "AUTHENTICATION_ERROR",
                                                    "field": "otp",
                                                    "message": "Invalid OTP. Please try again.",
                                                    "details": json!({
                                                        "mobile_no": mobile_no,
                                                        "session_token": session_token,
                                                        "otp": otp
                                                    }),
                                                    "timestamp": chrono::Utc::now().to_rfc3339(),
                                                    "socket_id": socket.id.to_string(),
                                                    "event": "otp:verification_failed"
                                                });

                                                // Store OTP verification failure event
                                                let _ = ds3.store_otp_verification_event(
                                                    &socket.id.to_string(),
                                                    mobile_no,
                                                    session_token,
                                                    otp,
                                                    false,
                                                    None,
                                                    None,
                                                    None
                                                ).await;

                                                let payload_doc = to_document(&error_response).unwrap_or_default();
                                                let _ = ds3.store_connection_error_event(
                                                    &socket.id.to_string(),
                                                    "INVALID_OTP",
                                                    "AUTHENTICATION_ERROR",
                                                    "otp",
                                                    "Invalid OTP. Please try again.",
                                                    payload_doc
                                                ).await;

                                                let _ = socket.emit("otp:verification_failed", error_response);
                                                info!("❌ OTP verification failed for mobile: {} (socket: {})", mobile_no, socket.id);
                                            }
                                            crate::database::models::OtpVerificationResult::Expired => {
                                                let error_response = json!({
                                                    "status": "error",
                                                    "error_code": "OTP_EXPIRED",
                                                    "error_type": "AUTHENTICATION_ERROR",
                                                    "field": "otp",
                                                    "message": "OTP has expired. Please request a new OTP.",
                                                    "details": json!({
                                                        "mobile_no": mobile_no,
                                                        "session_token": session_token,
                                                        "otp": otp
                                                    }),
                                                    "timestamp": chrono::Utc::now().to_rfc3339(),
                                                    "socket_id": socket.id.to_string(),
                                                    "event": "otp:verification_failed"
                                                });

                                                // Store OTP verification failure event
                                                let _ = ds3.store_otp_verification_event(
                                                    &socket.id.to_string(),
                                                    mobile_no,
                                                    session_token,
                                                    otp,
                                                    false,
                                                    None,
                                                    None,
                                                    None
                                                ).await;

                                                let payload_doc = to_document(&error_response).unwrap_or_default();
                                                let _ = ds3.store_connection_error_event(
                                                    &socket.id.to_string(),
                                                    "OTP_EXPIRED",
                                                    "AUTHENTICATION_ERROR",
                                                    "otp",
                                                    "OTP has expired. Please request a new OTP.",
                                                    payload_doc
                                                ).await;

                                                let _ = socket.emit("otp:verification_failed", error_response);
                                                info!("⏰ OTP expired for mobile: {} (socket: {})", mobile_no, socket.id);
                                            }
                                            crate::database::models::OtpVerificationResult::NotFound => {
                                                let error_response = json!({
                                                    "status": "error",
                                                    "error_code": "SESSION_NOT_FOUND",
                                                    "error_type": "AUTHENTICATION_ERROR",
                                                    "field": "session_token",
                                                    "message": "Invalid session. Please login again.",
                                                    "details": json!({
                                                        "mobile_no": mobile_no,
                                                        "session_token": session_token
                                                    }),
                                                    "timestamp": chrono::Utc::now().to_rfc3339(),
                                                    "socket_id": socket.id.to_string(),
                                                    "event": "otp:verification_failed"
                                                });

                                                let payload_doc = to_document(&error_response).unwrap_or_default();
                                                let _ = ds3.store_connection_error_event(
                                                    &socket.id.to_string(),
                                                    "SESSION_NOT_FOUND",
                                                    "AUTHENTICATION_ERROR",
                                                    "session_token",
                                                    "Invalid session. Please login again.",
                                                    payload_doc
                                                ).await;

                                                let _ = socket.emit("otp:verification_failed", error_response);
                                                info!("❌ Session not found for mobile: {} (socket: {})", mobile_no, socket.id);
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        let error_msg = e.to_string();
                                        let error_response = json!({
                                            "status": "error",
                                            "error_code": "OTP_VERIFICATION_ERROR",
                                            "error_type": "SYSTEM_ERROR",
                                            "field": "otp",
                                            "message": "OTP verification failed due to system error",
                                            "details": json!({
                                                "error": error_msg
                                            }),
                                            "timestamp": chrono::Utc::now().to_rfc3339(),
                                            "socket_id": socket.id.to_string(),
                                            "event": "otp:verification_failed"
                                        });
                                        let payload_doc = to_document(&error_response).unwrap_or_default();
                                        let _ = ds3.store_connection_error_event(
                                            &socket.id.to_string(),
                                            "OTP_VERIFICATION_ERROR",
                                            "SYSTEM_ERROR",
                                            "otp",
                                            "OTP verification failed due to system error",
                                            payload_doc
                                        ).await;
                                        let _ = socket.emit("otp:verification_failed", error_response);
                                        info!("❌ OTP verification system error for mobile: {} (socket: {}): {}", mobile_no, socket.id, error_msg);
                                    }
                                }
                            }
                            Err(error_details) => {
                                let error_response = json!({
                                    "status": "error",
                                    "error_code": error_details.code,
                                    "error_type": error_details.error_type,
                                    "field": error_details.field,
                                    "message": error_details.message,
                                    "details": error_details.details,
                                    "timestamp": chrono::Utc::now().to_rfc3339(),
                                    "socket_id": socket.id.to_string(),
                                    "event": "otp:verification_failed"
                                });
                                let payload_doc = to_document(&error_response).unwrap_or_default();
                                let _ = ds3.store_connection_error_event(
                                    &socket.id.to_string(),
                                    &error_details.code,
                                    &error_details.error_type,
                                    &error_details.field,
                                    &error_details.message,
                                    payload_doc
                                ).await;
                                let _ = socket.emit("otp:verification_failed", error_response);
                                info!("❌ OTP verification validation failed for socket {}: {:?}", socket.id, error_details);
                            }
                        }
                    }
                });

                // Handle user profile event
                let ds4 = data_service.clone();
                socket.on("set:profile", move |socket: SocketRef, Data::<serde_json::Value>(data)| {
                    let ds4 = ds4.clone();
                    async move {
                        // Use catch_unwind to prevent panics from crashing the server
                        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| async {
                            info!("👤 Received user profile request from {}: {:?}", socket.id, data);
                            match ValidationManager::validate_user_profile_data(&data) {
                                Ok(_) => {
                                    let mobile_no = data["mobile_no"].as_str().unwrap_or("unknown");
                                    let session_token = data["session_token"].as_str().unwrap_or("unknown");
                                    let full_name = data["full_name"].as_str().unwrap_or("unknown");
                                    let state = data["state"].as_str().unwrap_or("unknown");
                                    let referral_code = data["referral_code"].as_str().map(|s| s.to_string());
                                    let referred_by = data["referred_by"].as_str().map(|s| s.to_string());
                                    let profile_data = data.get("profile_data").cloned();
                                    
                                    // Verify session and mobile number
                                    let session_verified = ds4.verify_session_and_mobile(mobile_no, session_token).await;
                                    match session_verified {
                                        Ok(is_valid) => {
                                            if is_valid {
                                                // Get user information first
                                                let user_info = ds4.get_user_by_mobile(mobile_no).await;
                                                let (user_id, user_number) = match user_info {
                                                    Ok(Some(user)) => (user.user_id.clone(), user.user_number),
                                                    _ => {
                                                        // User not found, create new user
                                                        let (new_user_id, new_user_number) = ds4.register_new_user(
                                                            mobile_no,
                                                            data["device_id"].as_str().unwrap_or("unknown"),
                                                            data["fcm_token"].as_str().unwrap_or("unknown"),
                                                            data["email"].as_str()
                                                        ).await.unwrap_or(("unknown".to_string(), 0));
                                                        (new_user_id, new_user_number)
                                                    }
                                                };

                                                // Check if referral code already exists (if provided)
                                                let mut final_referral_code = referral_code;
                                                let referred_by_code = referred_by;
                                                
                                                if let Some(ref_code) = &final_referral_code {
                                                    let code_exists = ds4.check_referral_code_exists(ref_code).await;
                                                    match code_exists {
                                                        Ok(exists) => {
                                                            if exists {
                                                                let error_response = json!({
                                                                    "status": "error",
                                                                    "error_code": "REFERRAL_CODE_EXISTS",
                                                                    "error_type": "VALIDATION_ERROR",
                                                                    "field": "referral_code",
                                                                    "message": "Referral code already exists. Please choose a different one.",
                                                                    "details": json!({
                                                                        "referral_code": ref_code
                                                                    }),
                                                                    "timestamp": chrono::Utc::now().to_rfc3339(),
                                                                    "socket_id": socket.id.to_string(),
                                                                    "event": "connection_error"
                                                                });
                                                                let payload_doc = to_document(&error_response).unwrap_or_default();
                                                                let _ = ds4.store_connection_error_event(
                                                                    &socket.id.to_string(),
                                                                    "REFERRAL_CODE_EXISTS",
                                                                    "VALIDATION_ERROR",
                                                                    "referral_code",
                                                                    "Referral code already exists. Please choose a different one.",
                                                                    payload_doc
                                                                ).await;
                                                                let _ = socket.emit("connection_error", error_response);
                                                                info!("❌ User profile failed: Referral code already exists for mobile: {} (socket: {})", mobile_no, socket.id);
                                                                return;
                                                            }
                                                        }
                                                        Err(e) => {
                                                            let error_msg = e.to_string();
                                                            let error_response = json!({
                                                                "status": "error",
                                                                "error_code": "REFERRAL_CODE_CHECK_ERROR",
                                                                "error_type": "SYSTEM_ERROR",
                                                                "field": "referral_code",
                                                                "message": "Failed to check referral code due to system error",
                                                                "details": json!({
                                                                    "error": error_msg
                                                                }),
                                                                "timestamp": chrono::Utc::now().to_rfc3339(),
                                                                "socket_id": socket.id.to_string(),
                                                                "event": "connection_error"
                                                            });
                                                            let payload_doc = to_document(&error_response).unwrap_or_default();
                                                            let _ = ds4.store_connection_error_event(
                                                                &socket.id.to_string(),
                                                                "REFERRAL_CODE_CHECK_ERROR",
                                                                "SYSTEM_ERROR",
                                                                "referral_code",
                                                                "Failed to check referral code due to system error",
                                                                payload_doc
                                                            ).await;
                                                            let _ = socket.emit("connection_error", error_response);
                                                            info!("❌ User profile system error for mobile: {} (socket: {}): {}", mobile_no, socket.id, error_msg);
                                                            return;
                                                        }
                                                    }
                                                }
                                                
                                                // Generate referral code if not provided
                                                if final_referral_code.is_none() {
                                                    let generated_code = ds4.generate_unique_referral_code(mobile_no).await;
                                                    match generated_code {
                                                        Ok(code) => {
                                                            info!("Generated referral code: {} for mobile: {}", code, mobile_no);
                                                            final_referral_code = Some(code);
                                                        }
                                                        Err(e) => {
                                                            let error_msg = e.to_string();
                                                            let error_response = json!({
                                                                "status": "error",
                                                                "error_code": "REFERRAL_CODE_GENERATION_ERROR",
                                                                "error_type": "SYSTEM_ERROR",
                                                                "field": "referral_code",
                                                                "message": "Failed to generate referral code due to system error",
                                                                "details": json!({
                                                                    "error": error_msg
                                                                }),
                                                                "timestamp": chrono::Utc::now().to_rfc3339(),
                                                                "socket_id": socket.id.to_string(),
                                                                "event": "connection_error"
                                                            });
                                                            let payload_doc = to_document(&error_response).unwrap_or_default();
                                                            let _ = ds4.store_connection_error_event(
                                                                &socket.id.to_string(),
                                                                "REFERRAL_CODE_GENERATION_ERROR",
                                                                "SYSTEM_ERROR",
                                                                "referral_code",
                                                                "Failed to generate referral code due to system error",
                                                                payload_doc
                                                            ).await;
                                                            let _ = socket.emit("connection_error", error_response);
                                                            info!("❌ User profile system error for mobile: {} (socket: {}): {}", mobile_no, socket.id, error_msg);
                                                            return;
                                                        }
                                                    }
                                                }
                                                
                                                // Store user profile event
                                                let store_result = ds4.store_user_profile_event(
                                                    &socket.id.to_string(),
                                                    &user_id,
                                                    user_number,
                                                    mobile_no,
                                                    full_name
                                                ).await;
                                                
                                                if let Err(e) = store_result {
                                                    warn!("Failed to store user profile event: {}", e);
                                                }
                                                
                                                // Also update userregister collection
                                                let update_register_result = ds4.update_user_profile_in_register(
                                                    mobile_no,
                                                    Some(full_name.to_string()),
                                                    Some(state.to_string()),
                                                    final_referral_code.clone(),
                                                    referred_by_code.clone(),
                                                    profile_data.clone()
                                                ).await;
                                                
                                                match update_register_result {
                                                    Ok(_) => {
                                                        info!("✅ Successfully updated user profile in register for mobile: {}", mobile_no);
                                                    }
                                                    Err(e) => {
                                                        error!("❌ Failed to update user profile in register for mobile {}: {}", mobile_no, e);
                                                        // Continue with the flow even if update fails
                                                    }
                                                }
                                                
                                                // Prepare success response
                                                let success_response = json!({
                                                    "status": "success",
                                                    "message": "User profile updated successfully! 🎉",
                                                    "mobile_no": mobile_no,
                                                    "session_token": session_token,
                                                    "full_name": full_name,
                                                    "state": state,
                                                    "referral_code": final_referral_code,
                                                    "referred_by": referred_by_code,
                                                    "profile_data": profile_data,
                                                    "welcome_message": format!("Welcome {}! Your profile has been set up successfully.", full_name),
                                                    "next_steps": "You can now proceed to set your language preferences.",
                                                    "timestamp": chrono::Utc::now().to_rfc3339(),
                                                    "socket_id": socket.id.to_string(),
                                                    "event": "profile:set"
                                                });
                                                
                                                // Add error handling for emit
                                                match socket.emit("profile:set", success_response) {
                                                    Ok(_) => info!("✅ User profile successful for mobile: {} (name: {}, socket: {})", mobile_no, full_name, socket.id),
                                                    Err(e) => warn!("⚠️ Failed to emit profile:set for mobile: {} (socket: {}): {}", mobile_no, socket.id, e),
                                                }
                                                
                                                // Add a small delay to ensure the message is sent
                                                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                                            } else {
                                                let error_response = json!({
                                                    "status": "error",
                                                    "error_code": "INVALID_SESSION",
                                                    "error_type": "AUTHENTICATION_ERROR",
                                                    "field": "session_token",
                                                    "message": "Invalid session. Please login again.",
                                                    "details": json!({
                                                        "mobile_no": mobile_no,
                                                        "session_token": session_token
                                                    }),
                                                    "timestamp": chrono::Utc::now().to_rfc3339(),
                                                    "socket_id": socket.id.to_string(),
                                                    "event": "connection_error"
                                                });
                                                let payload_doc = to_document(&error_response).unwrap_or_default();
                                                let _ = ds4.store_connection_error_event(
                                                    &socket.id.to_string(),
                                                    "INVALID_SESSION",
                                                    "AUTHENTICATION_ERROR",
                                                    "session_token",
                                                    "Invalid session. Please login again.",
                                                    payload_doc
                                                ).await;
                                                let _ = socket.emit("connection_error", error_response);
                                                info!("❌ User profile failed: Invalid session for mobile: {} (socket: {})", mobile_no, socket.id);
                                            }
                                        }
                                        Err(e) => {
                                            let error_msg = e.to_string();
                                            let error_response = json!({
                                                "status": "error",
                                                "error_code": "SESSION_VERIFICATION_ERROR",
                                                "error_type": "SYSTEM_ERROR",
                                                "field": "session_token",
                                                "message": "Session verification failed due to system error",
                                                "details": json!({
                                                    "error": error_msg
                                                }),
                                                "timestamp": chrono::Utc::now().to_rfc3339(),
                                                "socket_id": socket.id.to_string(),
                                                "event": "connection_error"
                                            });
                                            let payload_doc = to_document(&error_response).unwrap_or_default();
                                            let _ = ds4.store_connection_error_event(
                                                &socket.id.to_string(),
                                                "SESSION_VERIFICATION_ERROR",
                                                "SYSTEM_ERROR",
                                                "session_token",
                                                "Session verification failed due to system error",
                                                payload_doc
                                            ).await;
                                            let _ = socket.emit("connection_error", error_response);
                                            info!("❌ User profile system error for mobile: {} (socket: {}): {}", mobile_no, socket.id, error_msg);
                                        }
                                    }
                                }
                                Err(error_details) => {
                                    let error_response = json!({
                                        "status": "error",
                                        "error_code": error_details.code,
                                        "error_type": error_details.error_type,
                                        "field": error_details.field,
                                        "message": error_details.message,
                                        "details": error_details.details,
                                        "timestamp": chrono::Utc::now().to_rfc3339(),
                                        "socket_id": socket.id.to_string(),
                                        "event": "connection_error"
                                    });
                                    let payload_doc = to_document(&error_response).unwrap_or_default();
                                    let _ = ds4.store_connection_error_event(
                                        &socket.id.to_string(),
                                        &error_details.code,
                                        &error_details.error_type,
                                        &error_details.field,
                                        &error_details.message,
                                        payload_doc
                                    ).await;
                                    let _ = socket.emit("connection_error", error_response);
                                    info!("❌ User profile validation failed for socket {}: {:?}", socket.id, error_details);
                                }
                            }
                        }));
                        
                        match result {
                            Ok(_) => {
                                // Handler completed successfully
                            }
                            Err(panic_info) => {
                                error!("💥 Panic in set:profile event handler for socket {}: {:?}", socket.id, panic_info);
                                let error_response = json!({
                                    "status": "error",
                                    "error_code": "INTERNAL_ERROR",
                                    "error_type": "SYSTEM_ERROR",
                                    "message": "Internal server error occurred",
                                    "timestamp": chrono::Utc::now().to_rfc3339(),
                                    "socket_id": socket.id.to_string(),
                                    "event": "connection_error"
                                });
                                let _ = socket.emit("connection_error", error_response);
                            }
                        }
                    }
                });

                // Handle language setting event
                let ds5 = data_service.clone();
                socket.on("set:language", move |socket: SocketRef, Data::<serde_json::Value>(data)| {
                    let ds5 = ds5.clone();
                    async move {
                        info!("🌐 Received language setting request from {}: {:?}", socket.id, data);
                        match ValidationManager::validate_language_setting_data(&data) {
                            Ok(_) => {
                                let mobile_no = data["mobile_no"].as_str().unwrap_or("unknown");
                                let session_token = data["session_token"].as_str().unwrap_or("unknown");
                                let language_code = data["language_code"].as_str().unwrap_or("unknown");
                                let language_name = data["language_name"].as_str().unwrap_or("unknown");
                                let region_code = data["region_code"].as_str();
                                let timezone = data["timezone"].as_str();
                                let user_preferences = data.get("user_preferences").cloned();
                                
                                // Verify session and mobile number
                                let session_verified = ds5.verify_session_and_mobile(mobile_no, session_token).await;
                                match session_verified {
                                    Ok(is_valid) => {
                                        if is_valid {
                                            // Get user information first
                                            let user_info = ds5.get_user_by_mobile(mobile_no).await;
                                            let (user_id, user_number) = match user_info {
                                                Ok(Some(user)) => (user.user_id.clone(), user.user_number),
                                                _ => {
                                                    // User not found, create new user
                                                    let (new_user_id, new_user_number) = ds5.register_new_user(
                                                        mobile_no,
                                                        data["device_id"].as_str().unwrap_or("unknown"),
                                                        data["fcm_token"].as_str().unwrap_or("unknown"),
                                                        data["email"].as_str()
                                                    ).await.unwrap_or(("unknown".to_string(), 0));
                                                    (new_user_id, new_user_number)
                                                }
                                            };

                                            // Store language setting event
                                            let store_result = ds5.store_language_setting_event(
                                                &socket.id.to_string(),
                                                &user_id,
                                                user_number,
                                                mobile_no,
                                                language_code,
                                                language_name,
                                                region_code,
                                                timezone,
                                                user_preferences.as_ref().unwrap_or(&serde_json::json!({}))
                                            ).await;
                                            
                                            if let Err(e) = store_result {
                                                warn!("Failed to store language setting event: {}", e);
                                            }
                                            
                                            // Also update userregister collection
                                            let update_register_result = ds5.update_user_language_in_register(
                                                mobile_no,
                                                Some(language_code.to_string()),
                                                Some(language_name.to_string()),
                                                region_code.map(|s| s.to_string()),
                                                timezone.map(|s| s.to_string()),
                                                user_preferences.clone().unwrap_or_else(|| serde_json::json!({}))
                                            ).await;
                                            
                                            match update_register_result {
                                                Ok(_) => {
                                                    info!("✅ Successfully updated user language in register for mobile: {}", mobile_no);
                                                }
                                                Err(e) => {
                                                    error!("❌ Failed to update user language in register for mobile {}: {}", mobile_no, e);
                                                    // Continue with the flow even if update fails
                                                }
                                            }
                                            
                                            // Prepare success response with localized messages
                                            let success_messages = get_localized_success_messages(language_code);
                                            let success_response = json!({
                                                "status": "success",
                                                "message": success_messages.welcome_message,
                                                "mobile_no": mobile_no,
                                                "session_token": session_token,
                                                "language_code": language_code,
                                                "language_name": language_name,
                                                "region_code": region_code,
                                                "timezone": timezone,
                                                "user_preferences": user_preferences.clone(),
                                                "localized_messages": json!({
                                                    "welcome": success_messages.welcome_message,
                                                    "setup_complete": success_messages.setup_complete,
                                                    "ready_to_play": success_messages.ready_to_play,
                                                    "next_steps": success_messages.next_steps
                                                }),
                                                "timestamp": chrono::Utc::now().to_rfc3339(),
                                                "socket_id": socket.id.to_string(),
                                                "event": "language:set"
                                            });
                                            
                                            // Add error handling for emit
                                            match socket.emit("language:set", success_response) {
                                                Ok(_) => info!("✅ Language setting successful for mobile: {} (language: {}, socket: {})", mobile_no, language_code, socket.id),
                                                Err(e) => warn!("⚠️ Failed to emit language:set for mobile: {} (socket: {}): {}", mobile_no, socket.id, e),
                                            }
                                            
                                            // Add a small delay to ensure the message is sent
                                            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                                        } else {
                                            let error_response = json!({
                                                "status": "error",
                                                "error_code": "INVALID_SESSION",
                                                "error_type": "AUTHENTICATION_ERROR",
                                                "field": "session_token",
                                                "message": "Invalid session. Please login again.",
                                                "details": json!({
                                                    "mobile_no": mobile_no,
                                                    "session_token": session_token
                                                }),
                                                "timestamp": chrono::Utc::now().to_rfc3339(),
                                                "socket_id": socket.id.to_string(),
                                                "event": "connection_error"
                                            });
                                            let payload_doc = to_document(&error_response).unwrap_or_default();
                                            let _ = ds5.store_connection_error_event(
                                                &socket.id.to_string(),
                                                "INVALID_SESSION",
                                                "AUTHENTICATION_ERROR",
                                                "session_token",
                                                "Invalid session. Please login again.",
                                                payload_doc
                                            ).await;
                                            let _ = socket.emit("connection_error", error_response);
                                            info!("❌ Language setting failed: Invalid session for mobile: {} (socket: {})", mobile_no, socket.id);
                                        }
                                    }
                                    Err(e) => {
                                        let error_msg = e.to_string();
                                        let error_response = json!({
                                            "status": "error",
                                            "error_code": "SESSION_VERIFICATION_ERROR",
                                            "error_type": "SYSTEM_ERROR",
                                            "field": "session_token",
                                            "message": "Session verification failed due to system error",
                                            "details": json!({
                                                "error": error_msg
                                            }),
                                            "timestamp": chrono::Utc::now().to_rfc3339(),
                                            "socket_id": socket.id.to_string(),
                                            "event": "connection_error"
                                        });
                                        let payload_doc = to_document(&error_response).unwrap_or_default();
                                        let _ = ds5.store_connection_error_event(
                                            &socket.id.to_string(),
                                            "SESSION_VERIFICATION_ERROR",
                                            "SYSTEM_ERROR",
                                            "session_token",
                                            "Session verification failed due to system error",
                                            payload_doc
                                        ).await;
                                        let _ = socket.emit("connection_error", error_response);
                                        info!("❌ Language setting system error for mobile: {} (socket: {}): {}", mobile_no, socket.id, error_msg);
                                    }
                                }
                            }
                            Err(error_details) => {
                                let error_response = json!({
                                    "status": "error",
                                    "error_code": error_details.code,
                                    "error_type": error_details.error_type,
                                    "field": error_details.field,
                                    "message": error_details.message,
                                    "details": error_details.details,
                                    "timestamp": chrono::Utc::now().to_rfc3339(),
                                    "socket_id": socket.id.to_string(),
                                    "event": "connection_error"
                                });
                                let payload_doc = to_document(&error_response).unwrap_or_default();
                                let _ = ds5.store_connection_error_event(
                                    &socket.id.to_string(),
                                    &error_details.code,
                                    &error_details.error_type,
                                    &error_details.field,
                                    &error_details.message,
                                    payload_doc
                                ).await;
                                let _ = socket.emit("connection_error", error_response);
                                info!("❌ Language setting validation failed for socket {}: {:?}", socket.id, error_details);
                            }
                        }
                    }
                });

                // Handle disconnect event
                socket.on("disconnect", |socket: SocketRef| async move {
                    info!("🔌 Client disconnected: {}", socket.id);
                });

                // Add heartbeat/ping handler to keep connection alive
                socket.on("ping", |socket: SocketRef| async move {
                    let pong_response = json!({
                        "status": "pong",
                        "timestamp": chrono::Utc::now().to_rfc3339(),
                        "socket_id": socket.id.to_string()
                    });
                    if let Err(e) = socket.emit("pong", pong_response) {
                        warn!("⚠️ Failed to send pong to socket {}: {}", socket.id, e);
                    }
                });

                // Add keepalive handler
                socket.on("keepalive", |socket: SocketRef| async move {
                    let keepalive_response = json!({
                        "status": "alive",
                        "timestamp": chrono::Utc::now().to_rfc3339(),
                        "socket_id": socket.id.to_string()
                    });
                    if let Err(e) = socket.emit("keepalive:ack", keepalive_response) {
                        warn!("⚠️ Failed to send keepalive ack to socket {}: {}", socket.id, e);
                    }
                });

                // Add connection health check handler
                socket.on("health_check", |socket: SocketRef| async move {
                    let health_response = json!({
                        "status": "healthy",
                        "timestamp": chrono::Utc::now().to_rfc3339(),
                        "socket_id": socket.id.to_string(),
                        "server_time": chrono::Utc::now().timestamp_millis(),
                        "connection_info": {
                            "protocol": "websocket",
                            "transport": "websocket"
                        }
                    });
                    if let Err(e) = socket.emit("health_check:ack", health_response) {
                        warn!("⚠️ Failed to send health check ack to socket {}: {}", socket.id, e);
                    }
                });
            }
        });
    }
} 