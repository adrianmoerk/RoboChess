/// converts a URSCRIPT command to a string that can be sent to the robot's gripper
/// # Arguments
/// * `command` - the URSCRIPT command to be converted
/// # Returns
/// * `String` - the converted command
pub fn generate_gripper_command(command: String) -> String {
  let result = 
              "def gripper():\n".to_string() +
              "#aliases for the gripper variable names\n" +
              "  ACT = 1\n" +
              "  GTO = 2\n" +
              "  ATR = 3\n" +
              "  ARD = 4\n" +
              "  FOR = 5\n" +
              "  SPE = 6\n" +
              "  OBJ = 7\n" +
              "  STA = 8\n" +
              "  FLT = 9\n" +
              "  POS = 10\n" +
              "  PRE = 11\n" +
              "  \n" +
              "  def rq_init_connection(gripper_sid=9, gripper_socket=\"1\"):\n" +
              "  \tsocket_open(\"127.0.0.1\",63352, gripper_socket)\n" +
              "  \tsocket_set_var(\"SID\", gripper_sid,  gripper_socket)\n" +
              "  \tack = socket_read_byte_list(3, gripper_socket)\n" +
              "  end\n" +
              "  \n" +
              "  def rq_set_sid(gripper_sid=9, gripper_socket=\"1\"):\n" +
              "          socket_set_var(\"SID\", gripper_sid,  gripper_socket)\n" +
              "          sync()\n" +
              "          return is_ack(socket_read_byte_list(3, gripper_socket))\n" +
              "  end\n" +
              "  \n" +
              "  def rq_activate(gripper_socket=\"1\"):\n" +
              "  \trq_gripper_act = 0\n" +
              "  \n" +
              "          if (not rq_is_gripper_activated(gripper_socket)):\n" +
              "              rq_reset(gripper_socket)\n" +
              "          end\n" +
              "  \n" +
              "  \trq_set_var(ACT,1, gripper_socket)\n" +
              "  end\n" +
              "  \n" +
              "  def rq_activate_and_wait(gripper_socket=\"1\"):\n" +
              "  \trq_activate(gripper_socket)\n" +
              "  \n" +
              "  \twhile(not rq_is_gripper_activated(gripper_socket)):\n" +
              "  \t\t# wait for activation completed\n" +
              "  \tend\n" +
              "  end\n" +
              "  \n" +
              "  def rq_stop(gripper_socket=\"1\"):\n" +
              "  \trq_set_var(GTO,0, gripper_socket)\n" +
              "  end\n" +
              "  \n" +
              "  def rq_reset(gripper_socket=\"1\"):\n" +
              "  \trq_gripper_act = 0\n" +
              "  \trq_obj_detect = 0\n" +
              "  \trq_mov_complete = 0\n" +
              "  \n" +
              "  \trq_set_var(ACT,0, gripper_socket)\n" +
              "  \trq_set_var(ATR,0, gripper_socket)\n" +
              "  end\n" +
              "  \n" +
              "  def rq_auto_release_open_and_wait(gripper_socket=\"1\"):\n" +
              "  \n" +
              "  \trq_set_var(ARD,0, gripper_socket)\n" +
              "  \trq_set_var(ACT,1, gripper_socket)\n" +
              "  \trq_set_var(ATR,1, gripper_socket)\n" +
              "  \n" +
              "  \tgFLT = rq_get_var(FLT, 2, gripper_socket)\n" +
              "  \n" +
              "  \twhile(not is_FLT_autorelease_completed(gFLT)):\n" +
              "  \t\tgFLT = rq_get_var(FLT, 2, gripper_socket)\n" +
              "  \tend\n" +
              "  end\n" +
              "  \n" +
              "  def rq_auto_release_close_and_wait(gripper_socket=\"1\"):\n" +
              "  \trq_set_var(ARD,1, gripper_socket)\n" +
              "  \trq_set_var(ACT,1, gripper_socket)\n" +
              "  \trq_set_var(ATR,1, gripper_socket)\n" +
              "  \n" +
              "  \tgFLT = rq_get_var(FLT, 2, gripper_socket)\n" +
              "  \n" +
              "  \twhile(not is_FLT_autorelease_completed(gFLT)):\n" +
              "  \t\tgFLT = rq_get_var(FLT, 2, gripper_socket)\n" +
              "  \tend\n" +
              "  end\n" +
              "  \n" +
              "  def rq_set_force(force, gripper_socket=\"1\"):\n" +
              "  \trq_set_var(FOR,force, gripper_socket)\n" +
              "  end\n" +
              "  \n" +
              "  def rq_set_speed(speed, gripper_socket=\"1\"):\n" +
              "  \trq_set_var(SPE,speed, gripper_socket)\n" +
              "  end\n" +
              "  \n" +
              "  def rq_open(gripper_socket=\"1\"):\n" +
              "  \trq_move(0, gripper_socket)\n" +
              "  end\n" +
              "  \n" +
              "  def rq_close(gripper_socket=\"1\"):\n" +
              "  \trq_move(255, gripper_socket)\n" +
              "  end\n" +
              "  \n" +
              "  def rq_open_and_wait(gripper_socket=\"1\"):\n" +
              "  \trq_move_and_wait(0, gripper_socket)\n" +
              "  end\n" +
              "  \n" +
              "  def rq_close_and_wait(gripper_socket=\"1\"):\n" +
              "  \trq_move_and_wait(255, gripper_socket)\n" +
              "  end\n" +
              "  \n" +
              "  def rq_move(pos, gripper_socket=\"1\"):\n" +
              "  \trq_mov_complete = 0\n" +
              "  \trq_obj_detect = 0\n" +
              "  \n" +
              "  \trq_set_pos(pos, gripper_socket)\n" +
              "  \trq_go_to(gripper_socket)\n" +
              "  end\n" +
              "  \n" +
              "  def rq_move_and_wait(pos, gripper_socket=\"1\"):\n" +
              "  \trq_move(pos, gripper_socket)\n" +
              "  \n" +
              "  \twhile (not rq_is_motion_complete(gripper_socket)):\n" +
              "  \t\t# wait for motion completed\n" +
              "  \t\tsleep(0.01)\n" +
              "  \t\tsync()\n" +
              "  \tend\n" +
              "  \n" +
              "  \t# following code used for compatibility with previous versions\n" +
              "  \trq_is_object_detected(gripper_socket)\n" +
              "  \n" +
              "  \tif (rq_obj_detect != 1):\n" +
              "  \t\trq_mov_complete = 1\n" +
              "  \tend\n" +
              "  end\n" +
              "  \n" +
              "  def rq_wait(gripper_socket=\"1\"):\n" +
              "          # Wait for the gripper motion to complete\n" +
              "          while (not rq_is_motion_complete(gripper_socket)):\n" +
              "  \t\t# wait for motion completed\n" +
              "  \t\tsleep(0.01)\n" +
              "  \t\tsync()\n" +
              "  \tend\n" +
              "  \n" +
              "  \t# following code used for compatibility with previous versions\n" +
              "  \trq_is_object_detected(gripper_socket)\n" +
              "  \n" +
              "  \tif (rq_obj_detect != 1):\n" +
              "  \t\trq_mov_complete = 1\n" +
              "  \tend\n" +
              "  end\n" +
              "  \n" +
              "  def rq_go_to(gripper_socket=\"1\"):\n" +
              "  \trq_set_var(GTO,1, gripper_socket)\n" +
              "  end\n" +
              "  \n" +
              "  # reset the rGTO to prevent movement and\n" +
              "  # set the position\n" +
              "  def rq_set_pos(pos, gripper_socket=\"1\"):\n" +
              "  \trq_set_var(GTO,0, gripper_socket)\n" +
              "  \n" +
              "  \trq_set_var(POS, pos, gripper_socket)\n" +
              "  \n" +
              "  \tgPRE = rq_get_var(PRE, 3, gripper_socket)\n" +
              "  \tpre = (gPRE[1] - 48)*100 + (gPRE[2] -48)*10 + gPRE[3] - 48\n" +
              "  \tsync()\n" +
              "  \twhile (pre != pos):\n" +
              "          rq_set_var(POS, pos, gripper_socket)\n" +
              "  \t\tgPRE = rq_get_var(PRE, 3, gripper_socket)\n" +
              "  \t\tpre = (gPRE[1] - 48)*100 + (gPRE[2] -48)*10 + gPRE[3] - 48\n" +
              "  \t\tsync()\n" +
              "  \tend\n" +
              "  end\n" +
              "  \n" +
              "  def rq_is_motion_complete(gripper_socket=\"1\"):\n" +
              "  \trq_mov_complete = 0\n" +
              "  \n" +
              "  \tgOBJ = rq_get_var(OBJ, 1, gripper_socket)\n" +
              "  \tsleep(0.01)\n" +
              "  \n" +
              "  \tif (is_OBJ_gripper_at_position(gOBJ)):\n" +
              "  \t\trq_mov_complete = 1\n" +
              "  \t\treturn True\n" +
              "  \tend\n" +
              "  \n" +
              "  \tif (is_OBJ_object_detected(gOBJ)):\n" +
              "  \t\trq_mov_complete = 1\n" +
              "  \t\treturn True\n" +
              "  \tend\n" +
              "  \n" +
              "  \treturn False\n" +
              "  \n" +
              "  end\n" +
              "  \n" +
              "  def rq_is_gripper_activated(gripper_socket=\"1\"):\n" +
              "  \tgSTA = rq_get_var(STA, 1, gripper_socket)\n" +
              "  \n" +
              "  \tif(is_STA_gripper_activated(gSTA)):\n" +
              "  \t\trq_gripper_act = 1\n" +
              "  \t\treturn True\n" +
              "  \telse:\n" +
              "  \t\trq_gripper_act = 0\n" +
              "  \t\treturn False\n" +
              "  \tend\n" +
              "  end\n" +
              "  \n" +
              "  def rq_is_object_detected(gripper_socket=\"1\"):\n" +
              "  \tgOBJ = rq_get_var(OBJ, 1, gripper_socket)\n" +
              "  \n" +
              "  \tif(is_OBJ_object_detected(gOBJ)):\n" +
              "  \t\trq_obj_detect = 1\n" +
              "  \t\treturn True\n" +
              "  \telse:\n" +
              "  \t\trq_obj_detect = 0\n" +
              "  \t\treturn False\n" +
              "  \tend\n" +
              "  end\n" +
              "  \n" +
              "  def rq_current_pos(gripper_socket=\"1\"):\n" +
              "  \trq_pos = socket_get_var(\"POS\",gripper_socket)\n" +
              "  \tsync()\n" +
              "      return rq_pos\n" +
              "  end\n" +
              "  \n" +
              "  def rq_print_gripper_fault_code(gripper_socket=\"1\"):\n" +
              "  \tgFLT = rq_get_var(FLT, 2, gripper_socket)\n" +
              "  \n" +
              "  \tif(is_FLT_no_fault(gFLT)):\n" +
              "  \t\ttextmsg(\"Gripper Fault : \", \"No Fault (0x00)\")\n" +
              "  \telif (is_FLT_action_delayed(gFLT)):\n" +
              "  \t\ttextmsg(\"Gripper Fault : \", \"Priority Fault: Action delayed, initialization must be completed prior to action (0x05)\")\n" +
              "  \telif (is_FLT_not_activated(gFLT)):\n" +
              "  \t\ttextmsg(\"Gripper Fault : \", \"Priority Fault: The activation must be set prior to action (0x07)\")\n" +
              "  \telif (is_FLT_autorelease_in_progress(gFLT)):\n" +
              "  \t\ttextmsg(\"Gripper Fault : \", \"Minor Fault: Automatic release in progress (0x0B)\")\n" +
              "  \telif (is_FLT_overcurrent(gFLT)):\n" +
              "  \t\ttextmsg(\"Gripper Fault : \", \"Minor Fault: Overcurrent protection tiggered (0x0E)\")\n" +
              "  \telif (is_FLT_autorelease_completed(gFLT)):\n" +
              "  \t\ttextmsg(\"Gripper Fault : \", \"Major Fault: Automatic release completed (0x0F)\")\n" +
              "  \telse:\n" +
              "  \t\ttextmsg(\"Gripper Fault : \", \"Unkwown Fault\")\n" +
              "  \tend\n" +
              "  end\n" +
              "  \n" +
              "  def rq_print_gripper_num_cycles(gripper_socket=\"1\"):\n" +
              "  \tsocket_send_string(\"GET NCY\",gripper_socket)\n" +
              "  \tsync()\n" +
              "  \tstring_from_server = socket_read_string(gripper_socket)\n" +
              "  \tsync()\n" +
              "  \n" +
              "  \tif(string_from_server == \"0\"):\n" +
              "  \t\ttextmsg(\"Gripper Cycle Number : \", \"Number of cycles is unreachable.\")\n" +
              "  \telse:\n" +
              "  \t\ttextmsg(\"Gripper Cycle Number : \", string_from_server)\n" +
              "  \tend\n" +
              "  end\n" +
              "  \n" +
              "  def rq_print_gripper_driver_state(gripper_socket=\"1\"):\n" +
              "  \tsocket_send_string(\"GET DST\",gripper_socket)\n" +
              "  \tsync()\n" +
              "  \tstring_from_server = socket_read_string(gripper_socket)\n" +
              "  \tsync()\n" +
              "  \n" +
              "  \tif(string_from_server == \"0\"):\n" +
              "  \t\ttextmsg(\"Gripper Driver State : \", \"RQ_STATE_INIT\")\n" +
              "  \telif(string_from_server == \"1\"):\n" +
              "  \t\ttextmsg(\"Gripper Driver State : \", \"RQ_STATE_LISTEN\")\n" +
              "  \telif(string_from_server == \"2\"):\n" +
              "  \t\ttextmsg(\"Gripper Driver State : \", \"RQ_STATE_READ_INFO\")\n" +
              "  \telif(string_from_server == \"3\"):\n" +
              "  \t\ttextmsg(\"Gripper Driver State : \", \"RQ_STATE_ACTIVATION\")\n" +
              "  \telse:\n" +
              "  \t\ttextmsg(\"Gripper Driver State : \", \"RQ_STATE_RUN\")\n" +
              "  \tend\n" +
              "  end\n" +
              "  \n" +
              "  def rq_print_gripper_serial_number():\n" +
              "  \t#socket_send_string(\"GET SNU\",gripper_socket)\n" +
              "  \t#sync()\n" +
              "  \t#string_from_server = socket_read_string(gripper_socket)\n" +
              "  \t#sync()\n" +
              "  \t#textmsg(\"Gripper Serial Number : \", string_from_server)\n" +
              "  end\n" +
              "  \n" +
              "  def rq_print_gripper_firmware_version(gripper_socket=\"1\"):\n" +
              "  \tsocket_send_string(\"GET FWV\",gripper_socket)\n" +
              "  \tsync()\n" +
              "  \tstring_from_server = socket_read_string(gripper_socket)\n" +
              "  \tsync()\n" +
              "  \ttextmsg(\"Gripper Firmware Version : \", string_from_server)\n" +
              "  end\n" +
              "  \n" +
              "  def rq_print_gripper_driver_version(gripper_socket=\"1\"):\n" +
              "  \tsocket_send_string(\"GET VER\",gripper_socket)\n" +
              "  \tsync()\n" +
              "  \tstring_from_server = socket_read_string(gripper_socket)\n" +
              "  \tsync()\n" +
              "  \ttextmsg(\"Gripper Driver Version : \", string_from_server)\n" +
              "  end\n" +
              "  \n" +
              "  def rq_print_gripper_probleme_connection(gripper_socket=\"1\"):\n" +
              "  \tsocket_send_string(\"GET PCO\",gripper_socket)\n" +
              "  \tsync()\n" +
              "  \tstring_from_server = socket_read_string(gripper_socket)\n" +
              "  \tsync()\n" +
              "  \tif (string_from_server == \"0\"):\n" +
              "  \t\ttextmsg(\"Gripper Connection State : \", \"No connection problem detected\")\n" +
              "  \telse:\n" +
              "  \t\ttextmsg(\"Gripper Connection State : \", \"Connection problem detected\")\n" +
              "  \tend\n" +
              "  end\n" +
              "  \n" +
              "  # Returns True if list_of_bytes is [3, 'a', 'c', 'k']\n" +
              "  def is_ack(list_of_bytes):\n" +
              "  \n" +
              "  \t# list length is not 3\n" +
              "  \tif (list_of_bytes[0] != 3):\n" +
              "  \t\treturn False\n" +
              "  \tend\n" +
              "  \n" +
              "  \t# first byte not is 'a'?\n" +
              "  \tif (list_of_bytes[1] != 97):\n" +
              "  \t\treturn False\n" +
              "  \tend\n" +
              "  \n" +
              "  \t# first byte not is 'c'?\n" +
              "  \tif (list_of_bytes[2] != 99):\n" +
              "  \t\treturn False\n" +
              "  \tend\n" +
              "  \n" +
              "  \t# first byte not is 'k'?\n" +
              "  \tif (list_of_bytes[3] != 107):\n" +
              "  \t\treturn False\n" +
              "  \tend\n" +
              "  \n" +
              "  \treturn True\n" +
              "  end\n" +
              "  \n" +
              "  # Returns True if list_of_bytes is not [3, 'a', 'c', 'k']\n" +
              "  def is_not_ack(list_of_bytes):\n" +
              "  \tif (is_ack(list_of_bytes)):\n" +
              "  \t\treturn False\n" +
              "  \telse:\n" +
              "  \t\treturn True\n" +
              "  \tend\n" +
              "  end\n" +
              "  \n" +
              "  def is_STA_gripper_activated (list_of_bytes):\n" +
              "  \n" +
              "  \t# list length is not 1\n" +
              "  \tif (list_of_bytes[0] != 1):\n" +
              "  \t\treturn False\n" +
              "  \tend\n" +
              "  \n" +
              "  \t# byte is '3'?\n" +
              "  \tif (list_of_bytes[1] == 51):\n" +
              "  \t\treturn True\n" +
              "  \tend\n" +
              "  \n" +
              "  \treturn False\n" +
              "  end\n" +
              "  \n" +
              "  # Returns True if list_of_byte is [1, '1'] or [1, '2']\n" +
              "  # Used to test OBJ = 0x1 or OBJ = 0x2\n" +
              "  def is_OBJ_object_detected (list_of_bytes):\n" +
              "  \n" +
              "  \t# list length is not 1\n" +
              "  \tif (list_of_bytes[0] != 1):\n" +
              "  \t\treturn False\n" +
              "  \tend\n" +
              "  \n" +
              "  \t# byte is '2'?\n" +
              "  \tif (list_of_bytes[1] == 50):\n" +
              "  \t\treturn True\n" +
              "  \tend\n" +
              "  \n" +
              "  \t# byte is '1'?\n" +
              "  \tif (list_of_bytes[1]  == 49):\n" +
              "  \t\treturn True\n" +
              "  \tend\n" +
              "  \n" +
              "  \treturn False\n" +
              "  \n" +
              "  end\n" +
              "  \n" +
              "  # Returns True if list_of_byte is [1, '3']\n" +
              "  # Used to test OBJ = 0x3\n" +
              "  def is_OBJ_gripper_at_position (list_of_bytes):\n" +
              "  \n" +
              "  \t# list length is not 1\n" +
              "  \tif (list_of_bytes[0] != 1):\n" +
              "  \t\treturn False\n" +
              "  \tend\n" +
              "  \n" +
              "  \t# byte is '3'?\n" +
              "  \tif (list_of_bytes[1] == 51):\n" +
              "  \t\treturn True\n" +
              "  \tend\n" +
              "  \n" +
              "  \treturn False\n" +
              "  end\n" +
              "  \n" +
              "  def is_not_OBJ_gripper_at_position (list_of_bytes):\n" +
              "  \n" +
              "  \tif (is_OBJ_gripper_at_position(list_of_bytes)):\n" +
              "  \t\treturn False\n" +
              "  \telse:\n" +
              "  \t\treturn True\n" +
              "  \tend\n" +
              "  end\n" +
              "  \n" +
              "  def is_FLT_no_fault(list_of_bytes):\n" +
              "  \n" +
              "  \t# list length is not 2\n" +
              "  \tif (list_of_bytes[0] != 2):\n" +
              "  \t\treturn False\n" +
              "  \tend\n" +
              "  \n" +
              "  \t# first byte is '0'?\n" +
              "  \tif (list_of_bytes[1] != 48):\n" +
              "  \t\treturn False\n" +
              "  \tend\n" +
              "  \n" +
              "  \t# second byte is '0'?\n" +
              "  \tif (list_of_bytes[2] != 48):\n" +
              "  \t\treturn False\n" +
              "  \tend\n" +
              "  \n" +
              "  \treturn True\n" +
              "  \n" +
              "  end\n" +
              "  \n" +
              "  def is_FLT_action_delayed(list_of_bytes):\n" +
              "  \n" +
              "  \t# list length is not 2\n" +
              "  \tif (list_of_bytes[0] != 2):\n" +
              "  \t\treturn False\n" +
              "  \tend\n" +
              "  \n" +
              "  \t# first byte is '0'?\n" +
              "  \tif (list_of_bytes[1] != 48):\n" +
              "  \t\treturn False\n" +
              "  \tend\n" +
              "  \n" +
              "  \t# second byte is '5'?\n" +
              "  \tif (list_of_bytes[2] != 53):\n" +
              "  \t\treturn False\n" +
              "  \tend\n" +
              "  \n" +
              "  \treturn True\n" +
              "  end\n" +
              "  \n" +
              "  def is_FLT_not_activated(list_of_bytes):\n" +
              "  \n" +
              "  \t# list length is not 2\n" +
              "  \tif (list_of_bytes[0] != 2):\n" +
              "  \t\treturn False\n" +
              "  \tend\n" +
              "  \n" +
              "  \t# first byte is '0'?\n" +
              "  \tif (list_of_bytes[1] != 48):\n" +
              "  \t\treturn False\n" +
              "  \tend\n" +
              "  \n" +
              "  \t# second byte is '7'?\n" +
              "  \tif (list_of_bytes[2] != 55):\n" +
              "  \t\treturn False\n" +
              "  \tend\n" +
              "  \n" +
              "  \treturn True\n" +
              "  end\n" +
              "  \n" +
              "  def is_FLT_autorelease_in_progress(list_of_bytes):\n" +
              "  \n" +
              "  \t# list length is not 2\n" +
              "  \tif (list_of_bytes[0] != 2):\n" +
              "  \t\treturn False\n" +
              "  \tend\n" +
              "  \n" +
              "  \t# first byte is '1'?\n" +
              "  \tif (list_of_bytes[1] != 49):\n" +
              "  \t\treturn False\n" +
              "  \tend\n" +
              "  \n" +
              "  \t# second byte is '1'?\n" +
              "  \tif (list_of_bytes[2] != 49):\n" +
              "  \t\treturn False\n" +
              "  \tend\n" +
              "  \n" +
              "  \treturn True\n" +
              "  \n" +
              "  end\n" +
              "  \n" +
              "  def is_FLT_overcurrent(list_of_bytes):\n" +
              "  \n" +
              "  \t# list length is not 2\n" +
              "  \tif (list_of_bytes[0] != 2):\n" +
              "  \t\treturn False\n" +
              "  \tend\n" +
              "  \n" +
              "  \t# first byte is '1'?\n" +
              "  \tif (list_of_bytes[1] != 49):\n" +
              "  \t\treturn False\n" +
              "  \tend\n" +
              "  \n" +
              "  \t# second byte is '4'?\n" +
              "  \tif (list_of_bytes[2] != 52):\n" +
              "  \t\treturn False\n" +
              "  \tend\n" +
              "  \n" +
              "  \treturn True\n" +
              "  \n" +
              "  end\n" +
              "  \n" +
              "  def is_FLT_autorelease_completed(list_of_bytes):\n" +
              "  \n" +
              "  \t# list length is not 2\n" +
              "  \tif (list_of_bytes[0] != 2):\n" +
              "  \t\treturn False\n" +
              "  \tend\n" +
              "  \n" +
              "  \t# first byte is '1'?\n" +
              "  \tif (list_of_bytes[1] != 49):\n" +
              "  \t\treturn False\n" +
              "  \tend\n" +
              "  \n" +
              "  \t# second byte is '5'?\n" +
              "  \tif (list_of_bytes[2] != 53):\n" +
              "  \t\treturn False\n" +
              "  \tend\n" +
              "  \n" +
              "  \treturn True\n" +
              "  \n" +
              "  end\n" +
              "  \n" +
              "  def rq_set_var(var_name, var_value, gripper_socket=\"1\"):\n" +
              "  \n" +
              "  \tsync()\n" +
              "  \tif (var_name == ACT):\n" +
              "  \t\tsocket_set_var(\"ACT\", var_value, gripper_socket)\n" +
              "  \telif (var_name == GTO):\n" +
              "  \t\tsocket_set_var(\"GTO\", var_value, gripper_socket)\n" +
              "  \telif (var_name == ATR):\n" +
              "  \t\tsocket_set_var(\"ATR\", var_value, gripper_socket)\n" +
              "  \telif (var_name == ARD):\n" +
              "  \t\tsocket_set_var(\"ARD\", var_value, gripper_socket)\n" +
              "  \telif (var_name == FOR):\n" +
              "  \t\tsocket_set_var(\"FOR\", var_value, gripper_socket)\n" +
              "  \telif (var_name == SPE):\n" +
              "  \t\tsocket_set_var(\"SPE\", var_value, gripper_socket)\n" +
              "  \telif (var_name == POS):\n" +
              "  \t\tsocket_set_var(\"POS\", var_value, gripper_socket)\n" +
              "  \telse:\n" +
              "  \tend\n" +
              "  \n" +
              "  \tsync()\n" +
              "  \tack = socket_read_byte_list(3, gripper_socket)\n" +
              "  \tsync()\n" +
              "  \n" +
              "  \twhile(is_not_ack(ack)):\n" +
              "  \n" +
              "  \t\ttextmsg(\"rq_set_var : retry\", \" ...\")\n" +
              "  \t\ttextmsg(\"rq_set_var : var_name = \", var_name)\n" +
              "  \t\ttextmsg(\"rq_set_var : var_value = \", var_value)\n" +
              "  \n" +
              "  \t\tif (ack[0] != 0):\n" +
              "  \t\t\ttextmsg(\"rq_set_var : invalid ack value = \", ack)\n" +
              "  \t\tend\n" +
              "  \n" +
              "  \t\tsocket_set_var(var_name , var_value,gripper_socket)\n" +
              "  \t\tsync()\n" +
              "  \t\tack = socket_read_byte_list(3, gripper_socket)\n" +
              "  \t\tsync()\n" +
              "  \tend\n" +
              "  end\n" +
              "  \n" +
              "  \n" +
              "  def rq_get_var(var_name, nbr_bytes, gripper_socket=\"1\"):\n" +
              "  \n" +
              "  \tif (var_name == FLT):\n" +
              "  \t\tsocket_send_string(\"GET FLT\",gripper_socket)\n" +
              "  \t\tsync()\n" +
              "  \telif (var_name == OBJ):\n" +
              "  \t\tsocket_send_string(\"GET OBJ\",gripper_socket)\n" +
              "  \t\tsync()\n" +
              "  \telif (var_name == STA):\n" +
              "  \t\tsocket_send_string(\"GET STA\",gripper_socket)\n" +
              "  \t\tsync()\n" +
              "  \telif (var_name == PRE):\n" +
              "  \t\tsocket_send_string(\"GET PRE\",gripper_socket)\n" +
              "  \t\tsync()\n" +
              "  \telse:\n" +
              "  \tend\n" +
              "  \n" +
              "  \tvar_value = socket_read_byte_list(nbr_bytes, gripper_socket)\n" +
              "  \tsync()\n" +
              "  \n" +
              "  \treturn var_value\n" +
              "  end\n" +
              "  \n" +
              "  ############################################\n" +
              "  # normalized functions (maps 0-100 to 0-255)\n" +
              "  ############################################\n" +
              "  def rq_set_force_norm(force_norm, gripper_socket=\"1\"):\n" +
              "      force_gripper = norm_to_gripper(force_norm)\n" +
              "      rq_set_force(force_gripper, gripper_socket)\n" +
              "  end\n" +
              "  \n" +
              "  def rq_set_speed_norm(speed_norm, gripper_socket=\"1\"):\n" +
              "      speed_gripper = norm_to_gripper(speed_norm)\n" +
              "      rq_set_speed(speed_gripper, gripper_socket)\n" +
              "  end\n" +
              "  \n" +
              "  def rq_move_norm(pos_norm, gripper_socket=\"1\"):\n" +
              "      pos_gripper = norm_to_gripper(pos_norm)\n" +
              "      rq_move(pos_gripper, gripper_socket)\n" +
              "  end\n" +
              "  \n" +
              "  def rq_move_and_wait_norm(pos_norm, gripper_socket=\"1\"):\n" +
              "      pos_gripper = norm_to_gripper(pos_norm)\n" +
              "      rq_move_and_wait(pos_gripper, gripper_socket)\n" +
              "  end\n" +
              "  \n" +
              "  def rq_set_pos_norm(pos_norm, gripper_socket=\"1\"):\n" +
              "      pos_gripper = norm_to_gripper(pos_norm)\n" +
              "      rq_set_pos(pos_gripper, gripper_socket)\n" +
              "  end\n" +
              "  \n" +
              "  \n" +
              "  def rq_current_pos_norm(gripper_socket=\"1\"):\n" +
              "      pos_gripper = rq_current_pos(gripper_socket)\n" +
              "      pos_norm = gripper_to_norm(pos_gripper)\n" +
              "      return pos_norm\n" +
              "  end\n" +
              "  \n" +
              "  def gripper_to_norm(value_gripper):\n" +
              "      value_norm = (value_gripper / 255) * 100\n" +
              "      return floor(value_norm)\n" +
              "  end\n" +
              "  \n" +
              "  def norm_to_gripper(value_norm):\n" +
              "      value_gripper = (value_norm / 100) * 255\n" +
              "      return ceil(value_gripper)\n" +
              "  end\n" +
              "  \n" +
              "  def rq_get_position():\n" +
              "      return rq_current_pos_norm()\n" +
              "  end\n" +
              "  #########################################\n" +
              "  rq_obj_detect = 0\n" +
              "  rq_init_connection(9, \"1\")\n" +
              "  connectivity_checked = [-1,-1,-1,-1]\n" +
              "  status_checked = [-1,-1,-1,-1]\n" +
              "  current_speed = [-1,-1,-1,-1]\n" +
              "  current_force = [-1,-1,-1,-1]\n" +
              "\n" +
              "  " + command.as_str() +
              "\n" +
              "end\n";
              result
}
