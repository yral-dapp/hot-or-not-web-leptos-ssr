use leptos::prelude::*;

#[component]
pub fn TokenConfettiSymbol(#[prop(into)] class: String) -> impl IntoView {
    view! {
        <svg class=class viewBox="0 0 254 175" fill="none" xmlns="http://www.w3.org/2000/svg">
            <g clip-path="url(#clip0_433_5816)">
                <path
                    d="M136.908 166.009C177.777 166.009 210.908 132.878 210.908 92.0089C210.908 51.1398 177.777 18.0089 136.908 18.0089C96.0391 18.0089 62.9082 51.1398 62.9082 92.0089C62.9082 132.878 96.0391 166.009 136.908 166.009Z"
                    fill="#FED056"
                ></path>
                <path
                    d="M136.908 152.307C170.21 152.307 197.206 125.311 197.206 92.0089C197.206 58.707 170.21 31.7104 136.908 31.7104C103.606 31.7104 76.6094 58.707 76.6094 92.0089C76.6094 125.311 103.606 152.307 136.908 152.307Z"
                    fill="#FEB635"
                ></path>
                <g filter="url(#filter0_d_433_5816)">
                    <path
                        fill-rule="evenodd"
                        clip-rule="evenodd"
                        d="M134.39 123.176C134.067 123.176 133.741 123.119 133.421 123.003C132.197 122.558 131.432 121.334 131.573 120.039L133.749 99.9426H117.075C116.03 99.9426 115.069 99.3674 114.576 98.4437C114.083 97.5201 114.14 96.4037 114.721 95.5339L137.07 62.1006C137.795 61.0154 139.167 60.5649 140.393 61.0154C141.62 61.4602 142.382 62.6842 142.241 63.9791L140.065 84.0759H156.742C157.787 84.0759 158.748 84.6511 159.241 85.5747C159.731 86.4984 159.677 87.6147 159.096 88.4846L136.744 121.918C136.209 122.723 135.313 123.176 134.39 123.176Z"
                        fill="white"
                    ></path>
                </g>
                <path
                    d="M197.206 92.0089C197.206 125.251 170.15 152.307 136.908 152.307C126.068 152.307 115.922 149.446 107.134 144.416C115.228 148.463 124.363 150.746 134.017 150.746C167.259 150.746 194.316 123.69 194.316 90.448C194.316 68.0456 182.002 48.4472 163.791 38.0698C183.591 47.9558 197.206 68.3925 197.206 92.0089ZM141.013 34.5433C152.893 34.5433 163.964 38.012 173.272 43.9378C163.155 36.3066 150.552 31.7394 136.908 31.7394C103.666 31.7394 76.6094 58.7667 76.6094 92.0378C76.6094 113.486 87.8828 132.391 104.822 143.028C90.1953 132.015 80.7141 114.527 80.7141 94.8417C80.7141 61.5995 107.77 34.5433 141.013 34.5433Z"
                    fill="#FC9924"
                ></path>
            </g>
            <path
                d="M12.1219 35.2554C12.4646 35.113 12.8556 35.1589 13.1614 35.3791L17.3589 38.2654C18.1228 38.7998 19.1601 38.1948 19.1026 37.2587L18.8099 32.1008C18.786 31.718 18.9435 31.3635 19.2417 31.1293L23.2389 27.9511C23.9767 27.374 23.7292 26.184 22.8312 25.9516L17.9336 24.6406C17.5659 24.5422 17.287 24.279 17.1566 23.9261L15.4259 19.0696C15.1075 18.1792 13.9252 18.0449 13.4265 18.8352L10.6856 23.1864C10.4917 23.5003 10.1479 23.6976 9.77804 23.7089L4.71155 23.8783C3.78229 23.9083 3.29223 25.011 3.88312 25.7424L7.10022 29.7345C7.33587 30.0252 7.41718 30.4187 7.3135 30.7742L5.91232 35.7437C5.65841 36.6469 6.52799 37.4674 7.40415 37.1203L12.1219 35.2554Z"
                fill="#2AAD52"
            ></path>
            <path
                d="M70.7113 32.1316C71.0669 31.9952 71.379 31.712 71.5482 31.3313C71.6307 31.1488 71.6767 30.9514 71.6837 30.7506C71.6907 30.5497 71.6586 30.3495 71.5891 30.1614C71.5196 29.9734 71.4141 29.8013 71.2789 29.655C71.1436 29.5088 70.9812 29.3914 70.801 29.3096C60.9329 24.7558 56.538 12.8639 61.0069 2.80804C61.3511 2.0335 61.0163 1.12754 60.2597 0.786326C59.5032 0.445108 58.6106 0.776755 58.2757 1.5477C53.1243 13.1395 58.1888 26.8433 69.5678 32.1023C69.9378 32.2651 70.3557 32.2681 70.7113 32.1316ZM24.635 83.5695C24.9906 83.4331 25.3027 83.15 25.4719 82.7692C29.9407 72.7134 41.6106 68.2349 51.4788 72.7888C52.2389 73.1395 53.1279 72.7984 53.4628 72.0274C53.7976 71.2565 53.4721 70.3469 52.7156 70.0057C41.3402 64.7563 27.8922 69.9171 22.7314 81.5125C22.3872 82.287 22.722 83.193 23.4786 83.5342C23.8615 83.703 24.2793 83.706 24.635 83.5695Z"
                fill="#FED056"
            ></path>
            <path
                d="M52.1711 35.8602C52.4706 35.7452 52.7254 35.5385 52.917 35.2473C53.3584 34.5334 53.1488 33.5903 52.4483 33.1405C43.4129 27.3415 39.4262 20.3127 40.2769 11.6563C40.3623 10.8177 39.755 10.0708 38.9321 9.98368C38.1092 9.8966 37.3762 10.5154 37.2907 11.354C36.3339 21.1401 40.9028 29.3392 50.853 35.7344C51.2584 35.9927 51.75 36.0218 52.1711 35.8602ZM43.1459 56.1688C43.567 56.0072 43.9141 55.6453 44.0609 55.1752C44.2962 54.3662 43.8433 53.5165 43.0494 53.2767C31.7748 49.8615 22.7173 51.7803 16.1214 58.972C15.5593 59.5906 15.5863 60.5602 16.1934 61.133C16.8005 61.7059 17.7485 61.6687 18.3141 61.0596C24.1375 54.6979 31.9514 53.1148 42.1885 56.2095C42.521 56.2997 42.8558 56.2801 43.1459 56.1688Z"
                fill="#E2017B"
            ></path>
            <path
                d="M44.4201 44.5516C44.7757 44.4151 45.0878 44.132 45.257 43.7513C45.3394 43.5687 45.3855 43.3713 45.3925 43.1705C45.3995 42.9697 45.3673 42.7694 45.2978 42.5814C45.2283 42.3933 45.1229 42.2212 44.9876 42.075C44.8524 41.9287 44.69 41.8113 44.5098 41.7296L34.1908 36.9676C33.4307 36.6169 32.5416 36.958 32.2068 37.729C31.8719 38.4999 32.1974 39.4095 32.954 39.7507L43.273 44.5126C43.6466 44.6851 44.0551 44.6916 44.4201 44.5516Z"
                fill="#FED056"
            ></path>
            <path
                d="M221.346 173.386C224.104 172.611 225.722 169.704 224.961 166.894C224.199 164.083 221.347 162.434 218.589 163.21C215.831 163.986 214.213 166.893 214.974 169.703C215.735 172.513 218.588 174.162 221.346 173.386Z"
                fill="#2AAD52"
            ></path>
            <path
                d="M231.852 145.038C230.666 145.372 229.4 145.295 228.349 145.242C226.334 145.121 225.392 145.164 224.81 146.079C224.358 146.787 223.427 146.986 222.734 146.536C222.039 146.076 221.834 145.129 222.286 144.421C223.88 141.921 226.558 142.077 228.518 142.192C230.532 142.312 231.475 142.269 232.057 141.355C232.705 140.337 232.269 139.149 231.506 137.292C230.687 135.302 229.662 132.821 231.203 130.389C232.798 127.89 235.475 128.046 237.435 128.16C239.45 128.281 240.392 128.238 240.974 127.323C241.426 126.615 242.357 126.416 243.05 126.866C243.745 127.326 243.94 128.275 243.498 128.981C241.913 131.478 239.226 131.325 237.275 131.208C235.261 131.087 234.319 131.13 233.736 132.045C233.089 133.062 233.525 134.25 234.287 136.108C235.106 138.097 236.131 140.578 234.591 143.01C233.835 144.163 232.874 144.751 231.852 145.038Z"
                fill="#E2017B"
            ></path>
            <path
                d="M236.088 102.696C238.949 102.696 241.268 100.333 241.268 97.4178C241.268 94.5026 238.949 92.1393 236.088 92.1393C233.227 92.1393 230.908 94.5026 230.908 97.4178C230.908 100.333 233.227 102.696 236.088 102.696Z"
                fill="#E2017B"
            ></path>
            <defs>
                <filter
                    id="filter0_d_433_5816"
                    x="114.242"
                    y="60.842"
                    width="46.332"
                    height="63.3339"
                    filterUnits="userSpaceOnUse"
                    color-interpolation-filters="sRGB"
                >
                    <feFlood flood-opacity="0" result="BackgroundImageFix"></feFlood>
                    <feColorMatrix
                        in="SourceAlpha"
                        type="matrix"
                        values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 127 0"
                        result="hardAlpha"
                    ></feColorMatrix>
                    <feOffset dx="1" dy="1"></feOffset>
                    <feComposite in2="hardAlpha" operator="out"></feComposite>
                    <feColorMatrix
                        type="matrix"
                        values="0 0 0 0 0.988235 0 0 0 0 0.6 0 0 0 0 0.141176 0 0 0 1 0"
                    ></feColorMatrix>
                    <feBlend
                        mode="normal"
                        in2="BackgroundImageFix"
                        result="effect1_dropShadow_433_5816"
                    ></feBlend>
                    <feBlend
                        mode="normal"
                        in="SourceGraphic"
                        in2="effect1_dropShadow_433_5816"
                        result="shape"
                    ></feBlend>
                </filter>
                <clipPath id="clip0_433_5816">
                    <rect
                        width="148"
                        height="148"
                        fill="white"
                        transform="translate(62.9082 18.0089)"
                    ></rect>
                </clipPath>
            </defs>
        </svg>
    }
}
