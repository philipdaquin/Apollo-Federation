use yew::{prelude::*, function_component, html, Html};

#[function_component(Footer)]
pub fn footer_component() -> Html {
    html! {
        <>
            <footer class="footer section">
                <div class="footer__container bd-grid">
                    <div class="footer__box">
                        <h3 class="footer__title">{"Apollo"}</h3>
                        <p class="footer__description">
                            {"New collection of shoes 2020."}
                        </p>
                    </div>

                    <div class="footer__box">
                        <h3 class="footer__title">{"EXPLORE"}</h3>
                        <p class="footer__description">
                            <ul>
                                <li><a href="#home" class="footer__link">{"Home"}</a></li>
                                <li><a href="#featured" class="footer__link">{"Featured"}</a></li>
                                <li><a href="#women" class="footer__link">{"Women"}</a></li>
                                <li><a href="#new" class="footer__link">{"new"}</a></li>
                            </ul>
                        </p>
                    </div>

                    <div class="footer__box">
                        <h3 class="footer__title">{"SUPPORT"}</h3>
                        <p class="footer__description">
                            <ul>
                                <li><a href="#home" class="footer__link">{"Products Help"}</a></li>
                                <li><a href="#featured" class="footer__link">{"Customer Care"}</a></li>
                                <li><a href="#women" class="footer__link">{"Authorised Service"}</a></li>
                            </ul>
                        </p>
                    </div>

                    <div class="footer__box">
                        <a href="#" class="footer__social"><i class="bx bxl-facebook"></i></a>
                        <a href="#" class="footer__social"><i class="bx bxl-instagram"></i></a>
                        <a href="#" class="footer__social"><i class="bx bxl-twitter"></i></a>
                        <a href="#" class="footer__social"><i class="bx bxl-google"></i></a>
                    </div>
                </div>

                <p class="footer__copy">{"©️ 2022 UNI. All rights reserve"}</p>
            </footer>
        </>
    }
}