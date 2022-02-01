use dioxus::prelude::*;
use dioxus_heroicons::{solid::Shape, IconButton};

fn main() {
    dioxus::web::launch(app)
}

fn app(cx: Scope) -> Element {
    let (cur, set_cur) = use_state(&cx, || None as Option<Shape>);

    cx.render(rsx! {
        div {
            margin_left: "2rem",
            margin_right: "2rem",
            div {
                margin_top: "1rem",
                margin_bottom: "1rem",
                div {
                    justify_content: "center",
                    align_items: "center",
                    margin: "auto",
                    h1 { "Select a Hero Icon" }
                    p {
                        match cur {
                            Some(icon) => rsx!{
                                "Icon copied to clipboard: "
                                IconButton {
                                    class: "some-css-class",
                                    title: "Delete it",
                                    size: 30,
                                    icon: *icon,
                                }
                            },
                            None => rsx!{ "No icon selected" },
                        }
                    }
                }
            }

            SHAPES.iter().map(|shape| rsx!{
                IconButton {
                    onclick: move |_| {
                        let formatted = get_formatted(shape);
                        let f = js_sys::Function::new_no_args(&format!("(function wr() {{ navigator.clipboard.writeText(`{formatted}`); }})()"));
                        f.call0(&wasm_bindgen::JsValue::NULL).unwrap();
                        set_cur(Some(*shape));
                    },
                    class: "some-css-class",
                    title: "Delete it",
                    size: 30,
                    icon: *shape,
                }
            })
        }
    })
}

fn get_formatted(shape: &Shape) -> String {
    let as_str = get_shape_name(shape);
    format!(
        r#"
        IconButton {{
            onclick: move |_| {{}},
            class: "some-css-class",
            title: "Delete it",
            disabled: disabled,
            size: 30,
            icon: Shape::{as_str},
        }}"#
    )
}

fn get_shape_name(shape: &Shape) -> &str {
    match shape {
        AcademicCap => "AcademicCap",
        Adjustments => "Adjustments",
        Annotation => "Annotation",
        Archive => "Archive",
        ArrowCircleDown => "ArrowCircleDown",
        ArrowCircleLeft => "ArrowCircleLeft",
        ArrowCircleRight => "ArrowCircleRight",
        ArrowCircleUp => "ArrowCircleUp",
        ArrowDown => "ArrowDown",
        ArrowLeft => "ArrowLeft",
        ArrowNarrowDown => "ArrowNarrowDown",
        ArrowNarrowLeft => "ArrowNarrowLeft",
        ArrowNarrowRight => "ArrowNarrowRight",
        ArrowNarrowUp => "ArrowNarrowUp",
        ArrowRight => "ArrowRight",
        ArrowSmDown => "ArrowSmDown",
        ArrowSmLeft => "ArrowSmLeft",
        ArrowSmRight => "ArrowSmRight",
        ArrowSmUp => "ArrowSmUp",
        ArrowUp => "ArrowUp",
        ArrowsExpand => "ArrowsExpand",
        AtSymbol => "AtSymbol",
        Backspace => "Backspace",
        BadgeCheck => "BadgeCheck",
        Ban => "Ban",
        Beaker => "Beaker",
        Bell => "Bell",
        BookOpen => "BookOpen",
        BookmarkAlt => "BookmarkAlt",
        Bookmark => "Bookmark",
        Briefcase => "Briefcase",
        Cake => "Cake",
        Calculator => "Calculator",
        Calendar => "Calendar",
        Camera => "Camera",
        Cash => "Cash",
        ChartBar => "ChartBar",
        ChartPie => "ChartPie",
        ChartSquareBar => "ChartSquareBar",
        ChatAlt2 => "ChatAlt2",
        ChatAlt => "ChatAlt",
        Chat => "Chat",
        CheckCircle => "CheckCircle",
        Check => "Check",
        ChevronDoubleDown => "ChevronDoubleDown",
        ChevronDoubleLeft => "ChevronDoubleLeft",
        ChevronDoubleRight => "ChevronDoubleRight",
        ChevronDoubleUp => "ChevronDoubleUp",
        ChevronDown => "ChevronDown",
        ChevronLeft => "ChevronLeft",
        ChevronRight => "ChevronRight",
        ChevronUp => "ChevronUp",
        Chip => "Chip",
        ClipboardCheck => "ClipboardCheck",
        ClipboardCopy => "ClipboardCopy",
        ClipboardList => "ClipboardList",
        Clipboard => "Clipboard",
        Clock => "Clock",
        CloudDownload => "CloudDownload",
        CloudUpload => "CloudUpload",
        Cloud => "Cloud",
        Code => "Code",
        Cog => "Cog",
        Collection => "Collection",
        ColorSwatch => "ColorSwatch",
        CreditCard => "CreditCard",
        CubeTransparent => "CubeTransparent",
        Cube => "Cube",
        CurrencyBangladeshi => "CurrencyBangladeshi",
        CurrencyDollar => "CurrencyDollar",
        CurrencyEuro => "CurrencyEuro",
        CurrencyPound => "CurrencyPound",
        CurrencyRupee => "CurrencyRupee",
        CurrencyYen => "CurrencyYen",
        CursorClick => "CursorClick",
        Database => "Database",
        DesktopComputer => "DesktopComputer",
        DeviceMobile => "DeviceMobile",
        DeviceTablet => "DeviceTablet",
        DocumentAdd => "DocumentAdd",
        DocumentDownload => "DocumentDownload",
        DocumentDuplicate => "DocumentDuplicate",
        DocumentRemove => "DocumentRemove",
        DocumentReport => "DocumentReport",
        DocumentSearch => "DocumentSearch",
        DocumentText => "DocumentText",
        Document => "Document",
        DotsCircleHorizontal => "DotsCircleHorizontal",
        DotsHorizontal => "DotsHorizontal",
        DotsVertical => "DotsVertical",
        Download => "Download",
        Duplicate => "Duplicate",
        EmojiHappy => "EmojiHappy",
        EmojiSad => "EmojiSad",
        ExclamationCircle => "ExclamationCircle",
        Exclamation => "Exclamation",
        ExternalLink => "ExternalLink",
        EyeOff => "EyeOff",
        Eye => "Eye",
        FastForward => "FastForward",
        Film => "Film",
        Filter => "Filter",
        FingerPrint => "FingerPrint",
        Fire => "Fire",
        Flag => "Flag",
        FolderAdd => "FolderAdd",
        FolderDownload => "FolderDownload",
        FolderOpen => "FolderOpen",
        FolderRemove => "FolderRemove",
        Folder => "Folder",
        Gift => "Gift",
        GlobeAlt => "GlobeAlt",
        Globe => "Globe",
        Hand => "Hand",
        Hashtag => "Hashtag",
        Heart => "Heart",
        Home => "Home",
        Identification => "Identification",
        InboxIn => "InboxIn",
        Inbox => "Inbox",
        InformationCircle => "InformationCircle",
        Key => "Key",
        Library => "Library",
        LightBulb => "LightBulb",
        LightningBolt => "LightningBolt",
        Link => "Link",
        LocationMarker => "LocationMarker",
        LockClosed => "LockClosed",
        LockOpen => "LockOpen",
        Login => "Login",
        Logout => "Logout",
        MailOpen => "MailOpen",
        Mail => "Mail",
        Map => "Map",
        MenuAlt1 => "MenuAlt1",
        MenuAlt2 => "MenuAlt2",
        MenuAlt3 => "MenuAlt3",
        MenuAlt4 => "MenuAlt4",
        Menu => "Menu",
        Microphone => "Microphone",
        MinusCircle => "MinusCircle",
        MinusSm => "MinusSm",
        Minus => "Minus",
        Moon => "Moon",
        MusicNote => "MusicNote",
        Newspaper => "Newspaper",
        OfficeBuilding => "OfficeBuilding",
        PaperAirplane => "PaperAirplane",
        PaperClip => "PaperClip",
        Pause => "Pause",
        PencilAlt => "PencilAlt",
        Pencil => "Pencil",
        PhoneIncoming => "PhoneIncoming",
        PhoneMissedCall => "PhoneMissedCall",
        PhoneOutgoing => "PhoneOutgoing",
        Phone => "Phone",
        Photograph => "Photograph",
        Play => "Play",
        PlusCircle => "PlusCircle",
        PlusSm => "PlusSm",
        Plus => "Plus",
        PresentationChartBar => "PresentationChartBar",
        PresentationChartLine => "PresentationChartLine",
        Printer => "Printer",
        Puzzle => "Puzzle",
        Qrcode => "Qrcode",
        QuestionMarkCircle => "QuestionMarkCircle",
        ReceiptRefund => "ReceiptRefund",
        ReceiptTax => "ReceiptTax",
        Refresh => "Refresh",
        Reply => "Reply",
        Rewind => "Rewind",
        Rss => "Rss",
        SaveAs => "SaveAs",
        Save => "Save",
        Scale => "Scale",
        Scissors => "Scissors",
        SearchCircle => "SearchCircle",
        Search => "Search",
        Selector => "Selector",
        Server => "Server",
        Share => "Share",
        ShieldCheck => "ShieldCheck",
        ShieldExclamation => "ShieldExclamation",
        ShoppingBag => "ShoppingBag",
        ShoppingCart => "ShoppingCart",
        SortAscending => "SortAscending",
        SortDescending => "SortDescending",
        Sparkles => "Sparkles",
        Speakerphone => "Speakerphone",
        Star => "Star",
        StatusOffline => "StatusOffline",
        StatusOnline => "StatusOnline",
        Stop => "Stop",
        Sun => "Sun",
        Support => "Support",
        SwitchHorizontal => "SwitchHorizontal",
        SwitchVertical => "SwitchVertical",
        Table => "Table",
        Tag => "Tag",
        Template => "Template",
        Terminal => "Terminal",
        ThumbDown => "ThumbDown",
        ThumbUp => "ThumbUp",
        Ticket => "Ticket",
        Translate => "Translate",
        Trash => "Trash",
        TrendingDown => "TrendingDown",
        TrendingUp => "TrendingUp",
        Truck => "Truck",
        Upload => "Upload",
        UserAdd => "UserAdd",
        UserCircle => "UserCircle",
        UserGroup => "UserGroup",
        UserRemove => "UserRemove",
        User => "User",
        Users => "Users",
        Variable => "Variable",
        VideoCamera => "VideoCamera",
        ViewBoards => "ViewBoards",
        ViewGridAdd => "ViewGridAdd",
        ViewGrid => "ViewGrid",
        ViewList => "ViewList",
        VolumeOff => "VolumeOff",
        VolumeUp => "VolumeUp",
        Wifi => "Wifi",
        XCircle => "XCircle",
        X => "X",
        ZoomIn => "ZoomIn",
        ZoomOut => "ZoomOut",
    }
}

use Shape::*;
const SHAPES: &[Shape] = &[
    AcademicCap,
    Adjustments,
    Annotation,
    Archive,
    ArrowCircleDown,
    ArrowCircleLeft,
    ArrowCircleRight,
    ArrowCircleUp,
    ArrowDown,
    ArrowLeft,
    ArrowNarrowDown,
    ArrowNarrowLeft,
    ArrowNarrowRight,
    ArrowNarrowUp,
    ArrowRight,
    ArrowSmDown,
    ArrowSmLeft,
    ArrowSmRight,
    ArrowSmUp,
    ArrowUp,
    ArrowsExpand,
    AtSymbol,
    Backspace,
    BadgeCheck,
    Ban,
    Beaker,
    Bell,
    BookOpen,
    BookmarkAlt,
    Bookmark,
    Briefcase,
    Cake,
    Calculator,
    Calendar,
    Camera,
    Cash,
    ChartBar,
    ChartPie,
    ChartSquareBar,
    ChatAlt2,
    ChatAlt,
    Chat,
    CheckCircle,
    Check,
    ChevronDoubleDown,
    ChevronDoubleLeft,
    ChevronDoubleRight,
    ChevronDoubleUp,
    ChevronDown,
    ChevronLeft,
    ChevronRight,
    ChevronUp,
    Chip,
    ClipboardCheck,
    ClipboardCopy,
    ClipboardList,
    Clipboard,
    Clock,
    CloudDownload,
    CloudUpload,
    Cloud,
    Code,
    Cog,
    Collection,
    ColorSwatch,
    CreditCard,
    CubeTransparent,
    Cube,
    CurrencyBangladeshi,
    CurrencyDollar,
    CurrencyEuro,
    CurrencyPound,
    CurrencyRupee,
    CurrencyYen,
    CursorClick,
    Database,
    DesktopComputer,
    DeviceMobile,
    DeviceTablet,
    DocumentAdd,
    DocumentDownload,
    DocumentDuplicate,
    DocumentRemove,
    DocumentReport,
    DocumentSearch,
    DocumentText,
    Document,
    DotsCircleHorizontal,
    DotsHorizontal,
    DotsVertical,
    Download,
    Duplicate,
    EmojiHappy,
    EmojiSad,
    ExclamationCircle,
    Exclamation,
    ExternalLink,
    EyeOff,
    Eye,
    FastForward,
    Film,
    Filter,
    FingerPrint,
    Fire,
    Flag,
    FolderAdd,
    FolderDownload,
    FolderOpen,
    FolderRemove,
    Folder,
    Gift,
    GlobeAlt,
    Globe,
    Hand,
    Hashtag,
    Heart,
    Home,
    Identification,
    InboxIn,
    Inbox,
    InformationCircle,
    Key,
    Library,
    LightBulb,
    LightningBolt,
    Link,
    LocationMarker,
    LockClosed,
    LockOpen,
    Login,
    Logout,
    MailOpen,
    Mail,
    Map,
    MenuAlt1,
    MenuAlt2,
    MenuAlt3,
    MenuAlt4,
    Menu,
    Microphone,
    MinusCircle,
    MinusSm,
    Minus,
    Moon,
    MusicNote,
    Newspaper,
    OfficeBuilding,
    PaperAirplane,
    PaperClip,
    Pause,
    PencilAlt,
    Pencil,
    PhoneIncoming,
    PhoneMissedCall,
    PhoneOutgoing,
    Phone,
    Photograph,
    Play,
    PlusCircle,
    PlusSm,
    Plus,
    PresentationChartBar,
    PresentationChartLine,
    Printer,
    Puzzle,
    Qrcode,
    QuestionMarkCircle,
    ReceiptRefund,
    ReceiptTax,
    Refresh,
    Reply,
    Rewind,
    Rss,
    SaveAs,
    Save,
    Scale,
    Scissors,
    SearchCircle,
    Search,
    Selector,
    Server,
    Share,
    ShieldCheck,
    ShieldExclamation,
    ShoppingBag,
    ShoppingCart,
    SortAscending,
    SortDescending,
    Sparkles,
    Speakerphone,
    Star,
    StatusOffline,
    StatusOnline,
    Stop,
    Sun,
    Support,
    SwitchHorizontal,
    SwitchVertical,
    Table,
    Tag,
    Template,
    Terminal,
    ThumbDown,
    ThumbUp,
    Ticket,
    Translate,
    Trash,
    TrendingDown,
    TrendingUp,
    Truck,
    Upload,
    UserAdd,
    UserCircle,
    UserGroup,
    UserRemove,
    User,
    Users,
    Variable,
    VideoCamera,
    ViewBoards,
    ViewGridAdd,
    ViewGrid,
    ViewList,
    VolumeOff,
    VolumeUp,
    Wifi,
    XCircle,
    X,
    ZoomIn,
    ZoomOut,
];
