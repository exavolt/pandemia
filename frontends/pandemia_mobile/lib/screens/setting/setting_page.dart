import 'package:flutter/material.dart';
import 'package:pandemia_mobile/blocs/settings/settings.dart';
import 'package:pandemia_mobile/blocs/settings/settings_bloc.dart';
import 'package:pandemia_mobile/user_repository/user_repository.dart';

class SettingScreen extends StatefulWidget {
  final SettingsBloc settingsBloc;
  SettingScreen({Key key, @required this.settingsBloc}) : super(key: key);

  @override
  _SettingScreenState createState() => _SettingScreenState(this.settingsBloc);
}

class _SettingScreenState extends State<SettingScreen> {
  final UserRepository _userRepo = UserRepository();
  final SettingsBloc settingsBloc;
  bool _onTap = false;
  bool _pushIsChecked = false;
  bool _petaIsChecked = false;
  bool _isBatuk = false;
  bool _isDemam = false;
  bool _isFlu = false;
  bool _isPusing = false;

  _SettingScreenState(this.settingsBloc){

    _pushIsChecked = _userRepo.currentUser.settings.enablePushNotif;
    _petaIsChecked = _userRepo.currentUser.settings.complaintMap;
    _isBatuk = _userRepo.currentUser.settings.hasCough;
    _isDemam = _userRepo.currentUser.settings.hasFever;
    _isFlu = _userRepo.currentUser.settings.hasFlu;
    _isPusing = _userRepo.currentUser.settings.hasHeadache;
  }

  @override
  Widget build(BuildContext context) {
    return Container(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.start,
        crossAxisAlignment: CrossAxisAlignment.start,
        children: <Widget>[
          Padding(
            padding: EdgeInsets.only(left: 15, top: 20, bottom: 5),
            child: Row(
              children: <Widget>[
                Checkbox(
                    value: _pushIsChecked,
                    onChanged: (value) {
                      setState(() {
                        _pushIsChecked = value;
                        settingsBloc.dispatch(SetSetting("enable_push_notif", _pushIsChecked ? "true": "false"));
                        _onTap = true;
                      });
                    }),
                // Text("[ ]"),
                Text(
                  "Push Notif",
                  style: TextStyle(
                    fontSize: 18,
                  ),
                )
              ],
            ),
          ),
          Padding(
            padding: EdgeInsets.only(bottom: 10),
            child: Center(
              child: InkWell(
                child: Text(
                  "Pilih hanya daerah tertentu saja",
                  style: TextStyle(
                    fontSize: 16,
                    color: Colors.blue,
                    decoration: TextDecoration.underline
                  ),
                ),
                onTap: _onTap
                    ? () {
                        print("=======> show filter screen");
                      }
                    : null,
              ),
            ),
          ),
          Divider(),
          Padding(
            padding: EdgeInsets.only(left: 15, top: 0, bottom: 0),
            child: Row(
              children: <Widget>[
                Checkbox(
                  value: _petaIsChecked,
                  onChanged: (value) {
                    _petaIsChecked == true
                        ? null
                        : showDialog(
                            context: context,
                            builder: (BuildContext context) {
                              return AlertDialog(
                                title: Row(
                                  children: <Widget>[
                                    Padding(
                                      padding: EdgeInsets.only(right: 10),
                                      child: Icon(
                                        Icons.info,
                                        color: Colors.purple[400],
                                      ),
                                    ),
                                    Text(
                                      "Informasi",
                                      style: TextStyle(color: Colors.purple),
                                    )
                                  ],
                                ),
                                content: Text(
                                    "Fitur ini memungkinkan anda untuk mendapatkan info daerah sekitar kita tentang pandemi Covid-19 ( Corona )"),
                                actions: <Widget>[
                                  Center(
                                    child: FlatButton(
                                      child: Text("OKE"),
                                      onPressed: () {
                                        Navigator.of(context).pop();
                                      },
                                    ),
                                  )
                                ],
                              );
                            });

                    setState(() {
                      _petaIsChecked = value;
                      settingsBloc.dispatch(SetSetting("complaint_map", _petaIsChecked ? "true": "false"));
                    });
                  },
                ),
                Text(
                  "Peta Keluhan",
                  style: TextStyle(
                    fontSize: 18,
                  ),
                )
              ],
            ),
          ),
          Padding(
            padding: EdgeInsets.only(left: 30, right: 30, bottom: 15),
            child: Text(
                "Menandai daerah keberadaan kita dengan keluhan kita, data hanya dalam bentuk " +
                    "statistik anonim (tidak ada data pribadi yang ditampilkan), " +
                    "fitur ini mempermudah kita dalam melakukan tracing.",
                maxLines: 5,
                style: TextStyle(fontSize: 16)),
          ),
          Padding(
            padding: EdgeInsets.only(left: 30, bottom: 15),
            child: Text(
              "Keluhan saya :",
              style: TextStyle(
                fontSize: 18,
              ),
            ),
          ),
          Padding(
            padding: EdgeInsets.only(left: 45),
            child: Column(
              children: <Widget>[
                Container(
                  height: 35,
                  child: Row(
                    children: <Widget>[
                      Checkbox(
                        value: _isBatuk,
                        onChanged: _petaIsChecked
                            ? (value) {
                                setState(() {
                                  _isBatuk = value;
                                  settingsBloc.dispatch(SetSetting("has_cough", _isBatuk ? "true": "false"));
                                });
                              }
                            : null,
                      ),
                      Text(
                        "Batuk",
                        style: TextStyle(
                          fontSize: 16,
                        ),
                      )
                    ],
                  ),
                ),
                Container(
                  height: 35,
                  child: Row(
                    children: <Widget>[
                      Checkbox(
                        value: _isDemam,
                        onChanged: _petaIsChecked
                            ? (value) {
                                setState(() {
                                  _isDemam = value;
                                  settingsBloc.dispatch(SetSetting("has_fever", _isDemam ? "true": "false"));
                                });
                              }
                            : null,
                      ),
                      Text(
                        "Demam",
                        style: TextStyle(
                          fontSize: 16,
                        ),
                      )
                    ],
                  ),
                ),
                Container(
                  height: 35,
                  child: Row(
                    children: <Widget>[
                      Checkbox(
                        value: _isFlu,
                        onChanged: _petaIsChecked
                            ? (value) {
                                setState(() {
                                  _isFlu = value;
                                  settingsBloc.dispatch(SetSetting("has_flu", _isFlu ? "true": "false"));
                                });
                              }
                            : null,
                      ),
                      Text(
                        "Flu",
                        style: TextStyle(
                          fontSize: 16,
                        ),
                      )
                    ],
                  ),
                ),
                Container(
                  height: 35,
                  child: Row(
                    children: <Widget>[
                      Checkbox(
                        value: _isPusing,
                        onChanged: _petaIsChecked
                            ? (value) {
                                setState(() {
                                  _isPusing = value;
                                  settingsBloc.dispatch(SetSetting("has_headache", _isPusing ? "true": "false"));
                                });
                              }
                            : null,
                      ),
                      Text(
                        "Pusing",
                        style: TextStyle(
                          fontSize: 16,
                        ),
                      )
                    ],
                  ),
                )
              ],
            ),
          )
        ],
      ),
    );
  }
}

// Widget _buildInfoDialog(BuildContext context) {
//   return new AlertDialog(
//     title: const Text("Info"),
//     content: new Container(
//       child: _buildText(context),
//     ),
//     actions: <Widget>[
//       new Center(
//         child: FlatButton(
//           child: const Text("OKE"),
//           onPressed: () {
//             Navigator.of(context).pop();
//           },
//         ),
//       )
//     ],
//   );
// }

// Widget _buildText(BuildContext context) {
//   return new RichText(
//       text: TextSpan(
//           text:
//               "Fitur ini memungkinkan anda untuk mendapatkan informasi Covid-19 ( Corona ) di daerah sekitar kita."));
// }
