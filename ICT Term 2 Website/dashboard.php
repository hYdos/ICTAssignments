<?php
function connectToDb() {
    $servername = "localhost";
    $username = "hvers5";
    $password = "bellbird";
    $dbname = "hvers5_Daymap2";

    // Create connection
    $conn = new mysqli($servername, $username, $password, $dbname);
    // Check connection
    if ($conn->connect_error) {
        die("Connection failed: " . $conn->connect_error);
    }
    return $conn;
}

function clearCookies() {
    if (isset($_SERVER['HTTP_COOKIE'])) {
        $cookies = explode(';', $_SERVER['HTTP_COOKIE']);
        foreach($cookies as $cookie) {
            $parts = explode('=', $cookie);
            $name = trim($parts[0]);
            setcookie($name, '', time()-1000);
            setcookie($name, '', time()-1000, '/');
        }
    }
}

function setUsrCookie($cookieName, $cookieVal) {
    setcookie($cookieName, $cookieVal, time() + (86400 * 30), "/"); // 30 Days should be short enough
}

function evalExpr($conn, $expr) {
    $sql = $expr;
    if ($result = $conn->query($sql)) {
        return @mysqli_fetch_assoc($result);
    } else {
        die("Error creating database: " . $conn->error);
    }
}

function login($username) {
    // I Really should be hashing & salting passwords
    // I should also filter the username and password to stop attacks on the DB
    clearCookies();
    clearCookies();
    $conn = connectToDb();
    $user = evalExpr($conn, "SELECT * FROM `Users` WHERE username=\"" . $username . "\"");
    $conn->close();

    setUsrCookie("firstName", $user["first_name"]);
    setUsrCookie("lastName", $user["last_name"]);
    setUsrCookie("email", $user["email"]);
}

session_start();
if ($_SERVER['REQUEST_METHOD'] == "POST") {
    switch ($_POST['actionType']) {
        case "LOGIN":
            login(ucfirst($_POST["user"]));
            break;

        case "REGISTER":
            $firstName = ucfirst($_POST['first_name']);
            $lastName = ucfirst($_POST['last_name']);
            $email = $_POST['email'];
            $password = $_POST['password'];

            $conn = connectToDb();
            $registration = evalExpr(
                $conn,
                "INSERT INTO `hvers5_Daymap2`.`Users` (`username`, `password`, `email`, `first_name`, `last_name`, `timetable`, `indicators`) VALUES ('$firstName', '$password', '$email', '$firstName', '$lastName', '{}', '{}');"
            );

            login($firstName);
            break;

        default:
            die("You got here without logging in somehow (actionType = " . $_POST['actionType'] . ")");
    }
}

if (!isset($_COOKIE["firstName"])) {
    die("Failed to set cookies during login");
}
?>
<!DOCTYPE html>
<html lang="en-us">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0, shrink-to-fit=no">
    <title>Dashboard</title>
    <link rel="stylesheet" href="assets/bootstrap/css/bootstrap.min.css">
    <link rel="stylesheet"
          href="https://fonts.googleapis.com/css?family=Nunito:200,200i,300,300i,400,400i,600,600i,700,700i,800,800i,900,900i">
    <link rel="stylesheet" href="https://use.fontawesome.com/releases/v5.12.0/css/all.css">
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/4.7.0/css/font-awesome.min.css">
    <link rel="stylesheet" href="assets/fonts/fontawesome5-overrides.min.css">
    <link rel="stylesheet" href="css/main.css">
</head>

<body id="page-top">
    <div id="wrapper">
        <nav class="navbar navbar-dark align-items-start sidebar sidebar-dark accordion sidebar-gradient p-0">
            <div class="container-fluid d-flex flex-column p-0"><a
                        class="navbar-brand d-flex justify-content-center align-items-center sidebar-brand m-0"
                        href="#">
                    <div class="sidebar-brand-icon rotate-n-15"></div>
                    <div class="sidebar-brand-text mx-3"><span>daymap 2</span></div>
                </a>
                <hr class="sidebar-divider my-0">
                <ul class="navbar-nav text-light" id="accordionSidebar">
                    <li class="nav-item"><a class="nav-link active" href="dashboard.php"><i
                                    class="fa fa-home"></i><span>Dashboard</span></a></li>
                    <li class="nav-item"></li>
                    <li class="nav-item"><a class="nav-link" href="timeTable.html"><i class="fas fa-table"></i><span>Time Table</span></a>
                    </li>
                    <li class="nav-item"><a class="nav-link" href="timeTable.html"><i
                                    class="far fa-plus-square"></i><span>If i get more time ill add more to this website.</span></a>
                    </li>
                </ul>
                <div class="text-center d-none d-md-inline"></div>
            </div>
        </nav>
        <div class="d-flex flex-column" id="content-wrapper" style="background: rgb(214,214,214);">
            <div class="flex-fill" id="content" style="color: rgb(172,173,183);">
                <nav class="navbar navbar-light navbar-expand bg-white shadow mb-4 topbar static-top"
                     style="height: 58px;margin: 9px;">
                    <div class="container-fluid">
                        <button class="btn btn-link d-md-none rounded-circle me-3" id="sidebarToggleTop" type="button">
                            <i class="fas fa-bars"></i></button>
                        <form class="d-none d-sm-inline-block me-auto ms-md-3 my-2 my-md-0 mw-100 navbar-search">
                            <div class="input-group"></div>
                        </form>
                        <ul class="navbar-nav flex-nowrap ms-auto">
                            <li class="nav-item dropdown d-sm-none no-arrow"><a class="dropdown-toggle nav-link"
                                                                                aria-expanded="false"
                                                                                data-bs-toggle="dropdown" href="#"><i
                                            class="fas fa-search"></i></a>
                                <div class="dropdown-menu dropdown-menu-end p-3 animated--grow-in"
                                     aria-labelledby="searchDropdown">
                                    <form class="me-auto navbar-search w-100">
                                        <div class="input-group"><input class="bg-light form-control border-0 small"
                                                                        type="text" placeholder="Search for ...">
                                            <div class="input-group-append">
                                                <button class="btn btn-primary py-0" type="button"><i
                                                            class="fas fa-search"></i></button>
                                            </div>
                                        </div>
                                    </form>
                                </div>
                            </li>
                            <li class="nav-item dropdown no-arrow mx-1"></li>
                            <li class="nav-item dropdown no-arrow mx-1">
                                <div class="nav-item dropdown no-arrow" style="box-shadow: inset 0px 0px;"><a
                                            class="dropdown-toggle nav-link" aria-expanded="false"
                                            data-bs-toggle="dropdown"
                                            href="#" style="transform: scale(1.34);"><i class="fa fa-bell"></i></a>
                                    <div class="dropdown-menu dropdown-menu-end dropdown-list animated--grow-in"
                                         style="box-shadow: 0px 0px 20px 4px;">
                                        <h6 class="dropdown-header">School Notices</h6><a
                                                class="dropdown-item d-flex align-items-center" href="#">
                                            <div class="dropdown-list-image me-3"><img
                                                        src="assets/img/misc/notice32.png"
                                                        style="transform: scale(0.73);">
                                            </div>
                                            <div class="fw-bold">
                                                <div class="text-truncate"><span>Cultural Performance P2</span></div>
                                                <p class="small text-gray-500 mb-0">School Notice - Monday</p>
                                            </div>
                                        </a><a class="dropdown-item d-flex align-items-center" href="#">
                                            <div class="dropdown-list-image me-3"><img
                                                        src="assets/img/misc/notice32.png"
                                                        style="transform: scale(0.73);">
                                            </div>
                                            <div class="fw-bold">
                                                <div class="text-truncate"><span>Another Room Change</span></div>
                                                <p class="small text-gray-500 mb-0">Maths 10E - Monday</p>
                                            </div>
                                        </a><a class="dropdown-item d-flex align-items-center" href="#">
                                            <div class="dropdown-list-image me-3"><img
                                                        src="assets/img/misc/notice32.png"
                                                        style="transform: scale(0.73);">
                                            </div>
                                            <div class="fw-bold">
                                                <div class="text-truncate"><span>So Many Room Changes</span></div>
                                                <p class="small text-gray-500 mb-0">Some Other Class - Friday</p>
                                            </div>
                                        </a><a class="dropdown-item d-flex align-items-center" href="#">
                                            <div class="dropdown-list-image me-3"><img
                                                        src="assets/img/misc/notice32.png"
                                                        style="transform: scale(0.73);">
                                            </div>
                                            <div class="fw-bold">
                                                <div class="text-truncate"><span>Test Test Test</span></div>
                                                <p class="small text-gray-500 mb-0">Lorem Ipsum</p>
                                            </div>
                                        </a><a class="dropdown-item text-center small text-gray-500" href="#">Show All
                                            Notices</a>
                                    </div>
                                </div>
                                <div class="shadow dropdown-list dropdown-menu dropdown-menu-end"
                                     aria-labelledby="alertsDropdown"></div>
                            </li>
                            <div class="d-none d-sm-block topbar-divider"></div>
                            <li class="nav-item dropdown no-arrow">
                                <div class="nav-item dropdown no-arrow"><a class="dropdown-toggle nav-link"
                                                                           aria-expanded="false"
                                                                           data-bs-toggle="dropdown" href="#"><span
                                                class="d-none d-lg-inline me-2 text-gray-600 small"><?php echo $_COOKIE["firstName"] . " " . $_COOKIE["lastName"] ?></span><img
                                                class="border rounded-circle img-profile"
                                                src="https://bellbirdparkssc.eq.daymap.net/DayMap/Images/profile-icon-grey.png"></a>
                                    <div class="dropdown-menu shadow dropdown-menu-end animated--grow-in"><a
                                                class="dropdown-item" href="#"><i
                                                    class="fas fa-user fa-sm fa-fw me-2 text-gray-400"></i>&nbsp;Profile</a><a
                                                class="dropdown-item" href="#"><i
                                                    class="fas fa-cogs fa-sm fa-fw me-2 text-gray-400"></i>&nbsp;Settings</a>
                                        <div class="dropdown-divider"></div>
                                        <a class="dropdown-item" href="#"><i
                                                    class="fas fa-sign-out-alt fa-sm fa-fw me-2 text-gray-400"></i>&nbsp;Logout</a>
                                    </div>
                                </div>
                            </li>
                        </ul>
                    </div>
                </nav>
                <div class="container-fluid">
                    <div class="d-sm-flex justify-content-between align-items-center mb-4">
                        <h3 class="text-dark mb-0">Dashboard</h3>
                    </div>
                    <div class="row" style="padding-bottom: 56px;">
                        <div class="col-md-6 col-xl-3 mb-4" style="width: 136.25px;">
                            <div class="card shadow border-start-primary py-2">
                                <div class="card-body">
                                    <div class="row align-items-center no-gutters">
                                        <div class="col me-2">
                                            <div class="text-uppercase text-primary fw-bold text-xs mb-1"><span>Attendance</span>
                                            </div>
                                            <div class="text-dark fw-bold h5 mb-0"><span>100%</span></div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                        <div class="col-md-6 col-xl-3 mb-4" style="width: 192.25px;">
                            <div class="card shadow border-start-primary py-2">
                                <div class="card-body">
                                    <div class="row align-items-center no-gutters">
                                        <div class="col me-2">
                                            <div class="text-uppercase text-primary fw-bold text-xs mb-1"><span
                                                        style="border-top-color: rgb(78, 115, 223);color: var(--bs-purple);">oneschool Reports</span>
                                            </div>
                                            <div class="text-dark fw-bold h5 mb-0"><span>0</span></div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                        <div class="col-md-6 col-xl-3 mb-4">
                            <div class="card shadow border-start-info py-2">
                                <div class="card-body">
                                    <div class="row align-items-center no-gutters">
                                        <div class="col me-2">
                                            <div class="text-uppercase text-info fw-bold text-xs mb-1"><span><strong>Work Completed</strong></span>
                                            </div>
                                            <div class="row g-0 align-items-center">
                                                <div class="col-auto">
                                                    <div class="text-dark fw-bold h5 mb-0 me-3"><span>50%</span></div>
                                                </div>
                                                <div class="col">
                                                    <div class="progress progress-sm">
                                                        <div class="progress-bar bg-info" aria-valuenow="50"
                                                             aria-valuemin="0" aria-valuemax="100" style="width: 50%;">
                                                            <span class="visually-hidden">50%</span></div>
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="row">
                        <div class="col-lg-7 col-xl-8">
                            <div class="card shadow mb-4">
                                <div class="card-header d-flex justify-content-between align-items-center">
                                    <h6 class="text-primary fw-bold m-0">Work</h6>
                                </div>
                                <div class="card-body">
                                    <ul>
                                        <li>Vet Cert II - due in 5 days&nbsp;</li>
                                        <li style="transform: perspective(0px);transform-style: preserve-3d;">English
                                            Creative Writing - due tomorrow
                                        </li>
                                        <li>Maths Report Final - due in 7 days</li>
                                    </ul>
                                </div>
                            </div>
                        </div>
                        <div class="col-lg-5 col-xl-4">
                            <div class="card shadow mb-4">
                                <div class="card-header d-flex justify-content-between align-items-center">
                                    <h6 class="text-primary fw-bold m-0">Status</h6>
                                </div>
                                <div class="card-body">
                                    <ul>
                                        <li style="border-top-color: rgb(155,210,154);color: rgb(55,159,53);">Completed.
                                            Not Marked
                                        </li>
                                        <li style="border-top-color: rgb(155,210,154);background: rgb(55,159,53);color: rgb(255,255,255);">
                                            Work Marked. Passed
                                        </li>
                                        <li style="background: #ffffff;">Work not received</li>
                                    </ul>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
    <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.0.1/dist/js/bootstrap.bundle.min.js"></script>
    <script src="js/sidebar.min.js"></script>
</body>

</html>