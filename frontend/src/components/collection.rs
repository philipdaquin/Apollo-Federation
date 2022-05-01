use yew::{prelude::*, function_component, html, Html};

#[function_component(ColletionObject)]
pub fn collection_component() -> Html {
    html! {
        <>
            <section class="collection section">
                <div class="collection__container bd-grid">
                    <div class="collection__card">
                        <div class="collection__data">
                            <h3 class="collection__name">{"Nike"}</h3>
                            <p class="collection__description">{"New Collection 2020"}</p>
                            <a href="#" class="button-light">{"Buy Now"}<i class="bx bx-right-arrow-alt button-icon"></i></a>
                        </div>
                        <img src="assets/img/collection1.png" alt="" class="collection__img"/>
                    </div>

                    <div class="collection__card">
                        <div class="collection__data">
                            <h3 class="collection__name">{"Adidas"}</h3>
                            <p class="collection__description">{"New Collection 2020"}</p>
                            <a href="#" class="button-light">{"Buy Now"}<i class="bx bx-right-arrow-alt button-icon"></i></a>
                        </div>
                        <img src="assets/img/collection2.png" alt="" class="collection__img"/>
                    </div>

                    <div class="collection__card">
                        <div class="collection__data">
                            <h3 class="collection__name">{"Name"}</h3>
                            <p class="collection__description">{"New Collection 2020"}</p>
                            <a href="#" class="button-light">{"Buy Now"}<i class="bx bx-right-arrow-alt button-icon"></i></a>
                        </div>
                        <img src="assets/img/collection4.png" alt="" class="collection__img"/>
                    </div>
                </div>
            </section>
        </>
    }
}